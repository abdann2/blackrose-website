use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Generate a random 25-characters-long case-sensitive subscription token.
pub fn generate_registration_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}
