use std::sync::Arc;

use crate::state::ApplicationState;
use axum::{extract::{MatchedPath, Request}, Router};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;
mod middleware;
pub mod request;
pub mod response;
mod v1;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url(
            "/v1/api-docs/openapi.json",
            crate::api::v1::ApiDoc::openapi(),
        ))
        .nest("/v1", v1::configure(state))
        .layer(axum::middleware::from_fn(middleware::trace::trace))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request:&Request|{
                let matched_path = request.extensions().get().map(MatchedPath::as_str);
                tracing::info_span!("http_request",method=?request.method(),path=?matched_path)
            })
        )
}
