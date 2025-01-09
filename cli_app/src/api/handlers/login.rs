use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    api::{
        request::login::LoginRequest,
        response::{TokenClaims, login::LoginResponse},
    },
    apperr::AppError,
    services::user::UserService,
    state::ApplicationState,
};

use crate::utils::password;
pub async fn login(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    //查询用户
    let user = match state
        .user_service
        .get_user_by_username(&payload.username)
        .await
    {
        Ok(user) => user,
        Err(_) => {
            return Err(AppError::from((
                StatusCode::UNAUTHORIZED,
                anyhow::anyhow!("Invalid username or password"),
            )));
        }
    };
    //校验密码
    let password = payload.password;
    let encryptpassword = password::encrypt_password(&password)?;
    if user.password != encryptpassword {
        return Err(AppError::from((
            StatusCode::UNAUTHORIZED,
            anyhow::anyhow!("password error"),
        )));
    }

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
