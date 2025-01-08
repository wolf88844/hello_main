use anyhow::Context;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(hello))
        .layer(tower_http::catch_panic::CatchPanicLayer::new());
    let addr = format!("0.0.0.0:{}", 3000);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("failed to bind to port")?;
    axum::serve(listener, app)
        .await
        .context("failed to serve")?;
    Ok(())
}

async fn hello() -> Result<(StatusCode, Json<Response>),AppError> {
    let response = Response {
        message: generate_message().context("failed to generate message")?,
    };
    Ok((StatusCode::OK, Json(response)))
}

fn generate_message() -> anyhow::Result<&'static str> {
    if rand::random() {
        anyhow::bail!("no message for you");
    }
    Ok("Hello World")
}

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

struct AppError(anyhow::Error);

impl From<anyhow::Error> for AppError{
    fn from(err: anyhow::Error) -> Self {
        Self(err)
    }
}

impl IntoResponse for AppError{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}