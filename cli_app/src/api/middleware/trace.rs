use std::{sync::Arc, time};

use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, MakeSpan, OnRequest, OnResponse,
};
use tracing::Level;

use crate::{apperr::AppError, state::ApplicationState};

pub async fn trace(
    State(_state): State<Arc<ApplicationState>>,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let span = DefaultMakeSpan::new().include_headers(true).make_span(&req);

    let _entered = span.enter();

    DefaultOnRequest::new()
        .level(Level::INFO)
        .on_request(&req, &span);

    let start = time::Instant::now();
    let response = next.run(req).await;

    let latency = start.elapsed();

    DefaultOnResponse::new()
        .level(Level::INFO)
        .latency_unit(tower_http::LatencyUnit::Micros)
        .on_response(&response, latency, &span);

    Ok(response)
}
