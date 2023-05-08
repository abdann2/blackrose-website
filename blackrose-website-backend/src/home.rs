use crate::{
    authentication::auth::{expire_in_five_hours, Claims},
    database::models::{
        NewUser, RegistrationQueryExtractor, RegistrationToken, User, UserCredentials,
        UserRegistrationCredentials,
    },
    email::RegistrationConfirmation,
    errors::{LoginError, RegistrationConfirmationError, RegistrationError},
    state::AppState,
    utils::generate_registration_token,
    BASE_URL, KEYS,
};
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse, Json, Response},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::result::Error as DieselError;
use diesel::{prelude::*, result::DatabaseErrorKind};
use diesel_async::{AsyncConnection, RunQueryDsl};
use jsonwebtoken::{encode, Header};
use scoped_futures::ScopedFutureExt;
use serde_json::{json, Value};

pub async fn root_handler() -> Response {
    Html(include_str!("../../blackrose-website-frontend/index.html")).into_response()
}
