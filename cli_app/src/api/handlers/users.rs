use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use utoipa::OpenApi;

use crate::{
    api::{
        request::user::{CreateUserRequest, UpdateUserRequest},
        response::user::{ListUserResponse, SingleUserResponse},
    },
    apperr::AppError,
    services::user::UserService,
    state::ApplicationState,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        create,
        list,
        get,
        get_by_username,
        update,
        delete,
    ),
    components(
        schemas(
            CreateUserRequest,
            UpdateUserRequest,
            ListUserResponse,
            SingleUserResponse,
        ),
    ),
    tags(
        (name = "Users", description = "User management operations"),
    )
)]
pub struct UsersApi;



#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "User created successfully", body = SingleUserResponse),
        (status = 400, description = "Bad request", body = AppError),
        (status = 500, description = "Internal server error", body = AppError),
    ),
    tag= "Users",
)]
pub async fn create(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<SingleUserResponse>, AppError> {
    let user = state.user_service.create_user(payload).await?;
    let response = SingleUserResponse { data: user };
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List of users", body = ListUserResponse),
        (status = 500, description = "Internal server error", body = AppError),
    ),
    tag="Users"
)]
pub async fn list(
    State(state): State<Arc<ApplicationState>>,
) -> Result<Json<ListUserResponse>, AppError> {
    let users = state.user_service.get_all_users().await?;
    let response = ListUserResponse { data: users };
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
        (status = 200, description = "User found", body = SingleUserResponse),
        (status = 404, description = "User not found", body = AppError),
        (status = 500, description = "Internal server error", body = AppError),
    ),
    params(
        ("id" = i64, Path, description = "User ID"),
    ),
    tag="Users"
)]
pub async fn get(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
) -> Result<Json<SingleUserResponse>, AppError> {
    let user = state.user_service.get_user_by_id(id).await;
    match user {
        Ok(user) => {
            let response = SingleUserResponse { data: user };
            Ok(Json(response))
        }
        Err(e) => Err(AppError::from((axum::http::StatusCode::NOT_FOUND, e))),
    }
}

#[utoipa::path(
    get,
    path = "/users/name/{username}",
    responses(
        (status = 200, description = "User found", body = SingleUserResponse),
        (status = 404, description = "User not found", body = AppError),
        (status = 500, description = "Internal server error", body = AppError),
    ),
    params(
        ("username" = String, Path, description = "User username"),
    ),
    tag="Users"
)]
pub async fn get_by_username(
    State(state): State<Arc<ApplicationState>>,
    Path(username): Path<String>,
) -> Result<Json<SingleUserResponse>, AppError> {
    let user = state.user_service.get_user_by_username(&username).await;
    match user {
        Ok(user) => {
            let response = SingleUserResponse { data: user };
            Ok(Json(response))
        }
        Err(e) => Err(AppError::from((axum::http::StatusCode::NOT_FOUND, e))),
    }
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = SingleUserResponse),
        (status = 400, description = "Bad request", body = AppError),
        (status = 404, description = "User not found", body = AppError),
        (status = 500, description = "Internal server error", body = AppError),
    ),
    params(
        ("id" = i64, Path, description = "User ID"),
    ),
    tag= "Users",
)]
pub async fn update(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<SingleUserResponse>, AppError> {
    let user = state.user_service.update_user(id, payload).await?;
    let response = SingleUserResponse { data: user };
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 404, description = "User not found", body = AppError),
        (status = 500, description = "Internal server error", body = AppError),
    ),
    params(
        ("id" = i64, Path, description = "User ID"),
    ),
    tag= "Users",
)]
pub async fn delete(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    state.user_service.delete_user(id).await?;
    Ok(())
}
