use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::adapters::general::general_responses::StopOperations;

pub fn hash_password(password: &str) -> Result<String, StopOperations> {
    let salt = SaltString::generate(&mut OsRng);
    let hasher = Argon2::default();
    let hash_pass = hasher.hash_password(password.as_bytes(), &salt);
    match hash_pass {
        Ok(res) => Ok(res.to_string()),
        Err(e) => Err(StopOperations::InternalMessage(format!(
            "Password Error : {}",
            e
        ))),
    }
}

pub fn verify_passwords(password: &str, original_hash: &str) -> Result<bool, StopOperations> {
    let pass_res = PasswordHash::new(original_hash);

    match pass_res {
        Ok(hash) => {
            let instance = Argon2::default();
            match instance.verify_password(password.as_bytes(), &hash) {
                Ok(()) => Ok(true),
                Err(_) => Ok(false),
            }
        }
        Err(err) => Err(StopOperations::InternalMessage(format!(
            "Password Error : {}",
            err
        ))),
    }
}
