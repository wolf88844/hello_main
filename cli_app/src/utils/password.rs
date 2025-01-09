use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

pub fn encrypt_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    if let Ok(hash) = argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash.to_string())
    } else {
        Err(anyhow::anyhow!("加密失败"))
    }
}
