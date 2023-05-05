use crate::{
    auth::{expire_in_five_hours, Claims},
    database::models::NewUser,
    database::models::{RegistrationToken, User, UserCredentials, UserRegistrationCredentials},
    errors::{LoginError, RegistrationError},
    state::AppState,
    utils::generate_registration_token,
    KEYS,
};
use axum::{
    extract::State,
    response::{Html, IntoResponse, Json, Response},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::result::Error as DieselError;
use diesel::{prelude::*, result::DatabaseErrorKind};
use diesel_async::RunQueryDsl;
use jsonwebtoken::{encode, Header};
use serde_json::{json, Value};

pub async fn root_handler() -> Response {
    Html(include_str!("../../blackrose-website-frontend/index.html")).into_response()
}

pub async fn login_handler(
    State(app_state): State<AppState>,
    Json(credentials): Json<UserCredentials>,
) -> Result<Json<Value>, LoginError> {
    // Checks if credentials are present
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(LoginError::MissingCredentials);
    }
    // Find the user associated with the email in the database
    use crate::database::schema::users::dsl::*;
    let mut conn = app_state.db.lock().await;
    let found_user: User = users
        .filter(email.eq(credentials.email.clone()))
        .first::<User>(&mut *conn)
        .await
        .map_err(|why| match why {
            DieselError::NotFound => LoginError::NotFound,
            _ => LoginError::InternalError,
        })?;
    // Release the lock
    drop(conn);
    // Check if the password is correct
    if verify(credentials.password, &found_user.password_hash)
        .map_err(|_| LoginError::InternalError)?
    {
        // Make the claim
        let claims = Claims {
            email: credentials.email,
            exp: expire_in_five_hours(),
        };

        // Make the token

        let token = encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|_| LoginError::TokenCreation)?;
        // Return token
        Ok(Json(json!({"access_token": token, "type": "Bearer"})))
    } else {
        // Wrong password for the found_user
        Err(LoginError::NotFound)
    }
}

pub async fn registration_handler(
    State(app_state): State<AppState>,
    Json(credentials): Json<UserRegistrationCredentials>,
) -> Result<Json<Value>, RegistrationError> {
    // Check for missing credentials
    if credentials.email.is_empty()
        || credentials.password.is_empty()
        || credentials.username.is_empty()
        || credentials.display_name.is_empty()
    {
        return Err(RegistrationError::MissingCredentials);
    }
    // Hash password
    let hashed_password =
        hash(credentials.password, DEFAULT_COST).map_err(|_| RegistrationError::InternalError)?;
    // Make unprivileged user, try to insert into table. If fails because of unique constraint, return UserAlreadyTaken
    let new_user = NewUser {
        email: credentials.email,
        username: credentials.username,
        display_name: credentials.display_name,
        password_hash: hashed_password,
        admin: false,
        registration_confirmed: false,
    };
    use crate::database::schema::users::dsl::*;
    let mut conn = app_state.db.lock().await;
    let inserted_user: User = new_user
        .insert_into(users)
        .get_result::<User>(&mut *conn)
        .await
        .map_err(|err| match err {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _info) => {
                RegistrationError::UserAlreadyTaken
            }
            _ => RegistrationError::InternalError,
        })?;
    // Make the registration token and store in the database. If an error occurs, send an internal error response
    let new_registration_token = RegistrationToken {
        user_id: inserted_user.id,
        registration_token: generate_registration_token(),
    };
    use crate::database::schema::registration_tokens::dsl::*;
    new_registration_token
        .insert_into(registration_tokens)
        .execute(&mut *conn)
        .await
        .map_err(|_| RegistrationError::InternalError)?;
    // Drop the connection
    drop(conn);
    // return a success message. TODO: consider making a struct for this.
    Ok(Json(json!({"msg": "registered successfully."})))
}
