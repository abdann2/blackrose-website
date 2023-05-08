use crate::{
    database::models::{
        NewUser, RegistrationQueryExtractor, RegistrationToken, User, UserRegistrationCredentials,
    },
    email::RegistrationConfirmation,
    errors::{RegistrationConfirmationError, RegistrationError},
    state::AppState,
    utils::generate_registration_token,
    BASE_URL,
};
use axum::{
    extract::{Query, State},
    response::{Html, Json},
};
use bcrypt::{hash, DEFAULT_COST};
use diesel::result::Error as DieselError;
use diesel::{prelude::*, result::DatabaseErrorKind};
use diesel_async::{AsyncConnection, RunQueryDsl};
use scoped_futures::ScopedFutureExt;
use serde_json::{json, Value};

pub async fn registration_handler(
    State(mut app_state): State<AppState>,
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
    // Hash the password
    let hashed_password =
        hash(credentials.password, DEFAULT_COST).map_err(|_| RegistrationError::InternalError)?;
    // Make a new unprivileged, registration unconfirmed user
    let new_user = NewUser {
        email: credentials.email.to_owned(),
        username: credentials.username.to_owned(),
        display_name: credentials.display_name,
        password_hash: hashed_password,
        admin: false,
        registration_confirmed: false,
    };
    use crate::database::schema::registration_tokens::dsl::*;
    use crate::database::schema::users::dsl::*;
    // Obtain the database connection and start a transaction to insert the new user and the registration token. If the user is already present, map the error to UserAlreadyTaken, other errors map to InternalError
    let generated_registration_token = generate_registration_token();
    // Cloning to satisfy borrow checker and the following move closure
    let registration_confirmation_link = format!(
        "{}/register/confirm?registration_token={}",
        BASE_URL.as_str(),
        &generated_registration_token
    );
    let mut conn = app_state.db.lock().await;
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        async move {
            let inserted_user: User = new_user
                .insert_into(users)
                .get_result::<User>(&mut *conn)
                .await?;
            let new_registration_token = RegistrationToken {
                user_id: inserted_user.id,
                registration_token: generated_registration_token,
            };
            new_registration_token
                .insert_into(registration_tokens)
                .execute(&mut *conn)
                .await?;
            Ok(())
        }
        .scope_boxed()
    })
    .await
    .map_err(|diesel_error| match diesel_error {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _info) => {
            RegistrationError::UserAlreadyTaken
        }
        _ => RegistrationError::InternalError,
    })?;
    // Drop the connection
    drop(conn);
    // Send the registration confirmation email
    app_state
        .email
        .send_registration_confirmation_email(
            &credentials.email,
            &credentials.username,
            &registration_confirmation_link,
        )
        .await
        .map_err(|_| RegistrationError::InternalError)?;
    // return a success message. TODO: consider making a struct for this.
    Ok(Json(json!({"msg": "registered successfully."})))
}

pub async fn registration_confirmation_handler<'a>(
    State(app_state): State<AppState>,
    query: Query<RegistrationQueryExtractor>,
) -> Result<Html<String>, RegistrationConfirmationError> {
    // First join users and registration_tokens together, updating the rows where we find our registration token. Then, we delete the registration_token from the registration_tokens table. Do this in a single transaction to be ACID compliant
    use crate::database::schema::registration_tokens::dsl::*;
    use crate::database::schema::users::dsl::*;
    let mut conn = app_state.db.lock().await;
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        async move {
            // Handle "not found" error possiblity after transaction
            let found_user_id: i32 = users
                .inner_join(registration_tokens)
                .filter(registration_token.eq(query.registration_token.to_owned()))
                .select(crate::database::schema::users::user_id)
                .first(&mut *conn)
                .await?;
            // Update that user
            diesel::update(users)
                .set(registration_confirmed.eq(true))
                .filter(crate::database::schema::users::user_id.eq(found_user_id))
                .execute(&mut *conn)
                .await?;
            // Delete the registration token from the registration_tokens table
            diesel::delete(registration_tokens.find(found_user_id))
                .execute(&mut *conn)
                .await?;
            Ok(())
        }
        .scope_boxed()
    })
    .await
    .map_err(|error| match error {
        // If join failed because no rows returned, return invalid token
        DieselError::NotFound => RegistrationConfirmationError::InvalidToken,
        _ => RegistrationConfirmationError::InternalError,
    })?;
    Ok(RegistrationConfirmation::success())
}
