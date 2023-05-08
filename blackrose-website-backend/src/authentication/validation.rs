use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserRegistrationCredentials {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegistrationQueryExtractor {
    pub registration_token: String,
}
