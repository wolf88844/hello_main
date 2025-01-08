use std::sync::Arc;

use crate::state::ApplicationState;

use super::handlers;
use axum::Router;
use axum::routing::get;
pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new().route("/hello", get(handlers::hello::hello).with_state(state))
}
