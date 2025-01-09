use serde::{Deserialize, Serialize};

pub mod login;
pub mod post;
pub mod user;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
