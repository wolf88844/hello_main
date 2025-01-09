use axum::{http::StatusCode, response::IntoResponse};
use utoipa::ToSchema;
#[derive(Debug, ToSchema)]
pub struct AppError {
    status_code: u16,
    err: String,
}

impl From<(StatusCode, anyhow::Error)> for AppError {
    fn from((status_code, err): (StatusCode, anyhow::Error)) -> Self {
        Self {
            status_code: status_code.as_u16(),
            err: err.to_string(),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            err: err.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            self.err,
        )
            .into_response()
    }
}
