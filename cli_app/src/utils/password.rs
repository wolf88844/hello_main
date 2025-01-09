use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString},
};
use rand::rngs::OsRng;

pub fn encrypt_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    if let Ok(hash) = argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash.to_string())
    } else {
        Err(anyhow::anyhow!("加密失败"))
    }
}

pub fn validate_password(password: &str, hash: &str) -> anyhow::Result<()> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash).map_err(|e| anyhow::anyhow!("解析hash失败:{}", e))?;
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| anyhow::anyhow!("验证失败"))
}
