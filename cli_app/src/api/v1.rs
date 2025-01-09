use std::sync::Arc;

use crate::state::ApplicationState;

use super::handlers;
use super::middleware::auth::auth;
use super::middleware::trace::trace;
use axum::routing::{delete, get, post, put};
use axum::{Router, middleware};
use utoipa::OpenApi;
pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route(
            "/hello",
            get(handlers::hello::hello).with_state(state.clone()),
        )
        .route(
            "/posts",
            post(handlers::posts::create)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
        .route(
            "/posts",
            get(handlers::posts::list)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(state.clone(), trace)),
        )
        .route(
            "/posts/{id}",
            get(handlers::posts::get).with_state(state.clone()),
        )
        .route(
            "/posts/slug/{name}",
            get(handlers::posts::get_by_slug).with_state(state.clone()),
        )
        .route(
            "/posts/{id}",
            put(handlers::posts::update).with_state(state.clone()),
        )
        .route(
            "/posts/{id}",
            delete(handlers::posts::delete).with_state(state.clone()),
        )
        .route(
            "/users",
            post(handlers::users::create).with_state(state.clone()),
        )
        .route(
            "/users",
            get(handlers::users::list).with_state(state.clone()),
        )
        .route(
            "/users/{id}",
            get(handlers::users::get).with_state(state.clone()),
        )
        .route(
            "/users/name/{name}",
            get(handlers::users::get_by_username).with_state(state.clone()),
        )
        .route(
            "/users/{id}",
            put(handlers::users::update).with_state(state.clone()),
        )
        .route(
            "/users/{id}",
            delete(handlers::users::delete).with_state(state.clone()),
        )
        .route("/login", post(handlers::login::login))
        .with_state(state.clone())
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::hello::hello,
        handlers::posts::create,
        handlers::posts::list,
        handlers::posts::get,
        handlers::posts::get_by_slug,
        handlers::posts::update,
        handlers::posts::delete,
        handlers::users::create,
        handlers::users::list,
        handlers::users::get,
        handlers::users::update,
        handlers::users::delete,
        handlers::login::login,
    ),
    components(
        schemas(
            crate::request::post::CreatePostRequest,
            crate::response::post::ListPostResponse,
            crate::response::post::SinglePostResponse,
            crate::request::post::CreatePostRequest,
            crate::request::user::CreateUserRequest,
            crate::response::user::ListUserResponse,
            crate::response::user::SingleUserResponse,
            crate::request::user::UpdateUserRequest,
            crate::request::login::LoginRequest,
            crate::response::login::LoginResponse,
        )
    ),
    tags(
        (name="hello",description="hello world"),
        (name="posts",description="posts api"),
        (name="users",description="users api"),
        (name="login",description="login api"),
    ),
    servers(
        (url="/v1",description="v1")
    ),
)]
pub struct ApiDoc;
