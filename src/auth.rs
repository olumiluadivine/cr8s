use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;

use crate::models::User;

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn authorize_user(user: &User, credentials: &Credentials) -> Result<String, String> {
    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    let a = Argon2::default()
        .verify_password(credentials.password.as_bytes(), &parsed_hash)
        .is_ok();
    if a {
        Ok(rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(128)
            .map(char::from)
            .collect())
    } else {
        Err("Invalid Login Attempt".to_string())
    }
}

pub fn hash_password(p: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default().hash_password(p.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}
