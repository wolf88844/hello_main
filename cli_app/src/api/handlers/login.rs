use std::sync::Arc;

use axum::{Json, extract::State};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    api::{
        request::login::LoginRequest,
        response::{TokenClaims, login::LoginResponse},
    },
    apperr::AppError,
    state::ApplicationState,
};

pub async fn login(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let timeout = state.settings.load().token_timeout_seconds.unwrap_or(3600);

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::seconds(timeout)).timestamp() as usize;
    let claims = TokenClaims {
        sub: payload.username.clone(),
        exp,
        iat,
    };

    let secret = state
        .settings
        .load()
        .token_secret
        .clone()
        .unwrap_or("secret".to_string());

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    let response = LoginResponse {
        status: "success".to_string(),
        token,
    };
    Ok(Json(response))
}
