use std::sync::Arc;

use axum::{extract::State, http::StatusCode};

use crate::state::ApplicationState;

#[utoipa::path(
    get,
    path = "/hello",
    tag = "Hello",
    responses(
        (status = 200, description = "Hello world!", body = String),
    ),
)]
pub async fn hello(State(state): State<Arc<ApplicationState>>) -> Result<String, StatusCode> {
    Ok(format!(
        "\nhello world! Using configuration from {}\n\n",
        state
            .settings
            .load()
            .config_info
            .location
            .clone()
            .unwrap_or("missing config location".to_string())
    )
    .to_string())
}
