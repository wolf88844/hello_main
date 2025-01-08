use std::sync::Arc;

use crate::state::ApplicationState;

use super::handlers;
use axum::Router;
use axum::routing::{delete, get, post, put};
pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route(
            "/hello",
            get(handlers::hello::hello).with_state(state.clone()),
        )
        .route(
            "/posts",
            post(handlers::posts::create).with_state(state.clone()),
        )
        .route(
            "/post",
            get(handlers::posts::list).with_state(state.clone()),
        )
        .route(
            "/posts/:id",
            get(handlers::posts::get).with_state(state.clone()),
        )
        .route(
            "/posts/:name",
            get(handlers::posts::get_by_slug).with_state(state.clone()),
        )
        .route(
            "/posts/:id",
            put(handlers::posts::update).with_state(state.clone()),
        )
        .route(
            "/posts/:id",
            delete(handlers::posts::delete).with_state(state.clone()),
        )
}
