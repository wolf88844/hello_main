use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::IntoResponse,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{api::response::TokenClaims, apperr::AppError, state::ApplicationState};

pub async fn auth(
    State(state): State<Arc<ApplicationState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            auth_value
                .strip_prefix("Bearer ")
                .map(|stripped| stripped.to_owned())
        });

    let token = token.ok_or_else(|| {
        AppError::from((
            StatusCode::UNAUTHORIZED,
            anyhow::anyhow!("Missing authorization header"),
        ))
    })?;

    let secret = state
        .settings
        .load()
        .token_secret
        .clone()
        .unwrap_or("secret".to_string());

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|err| {
        AppError::from((
            StatusCode::UNAUTHORIZED,
            anyhow::anyhow!("Invalid token: {}", err),
        ))
    })?
    .claims;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
