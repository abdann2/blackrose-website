use crate::{
    authentication::{
        auth::{expire_in_five_hours, Claims},
        validation::UserCredentials,
    },
    database::models::User,
    errors::LoginError,
    state::AppState,
    KEYS,
};
use axum::{extract::State, response::Json};
use bcrypt::verify;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::RunQueryDsl;
use jsonwebtoken::{encode, Header};
use serde_json::{json, Value};

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
    // Check if the user has confirmed their registration
    if !found_user.registration_confirmed {
        return Err(LoginError::UnconfirmedUserRegistration);
    }
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
