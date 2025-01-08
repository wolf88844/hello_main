use axum::{http::StatusCode, response::IntoResponse};

pub struct AppError{
status_code:StatusCode,
err:anyhow::Error,
}

impl From<(StatusCode,anyhow::Error)> for AppError {
    fn from((status_code,err):(StatusCode,anyhow::Error)) -> Self {
        Self {
            status_code,
            err,
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err:anyhow::Error) -> Self {
        Self {
            status_code:StatusCode::INTERNAL_SERVER_ERROR,
            err,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, self.err.to_string()).into_response()
    }
}
