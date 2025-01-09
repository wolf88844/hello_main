use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    api::request::CreatePostRequest, api::request::UpdatePostRequest,
    api::response::ListPostResponse, api::response::SinglePostResponse, api::response::TokenClaims,
    apperr::AppError, model::Post, services::post::PostService, state::ApplicationState,
};

#[utoipa::path(
    post,
    path = "/posts",
    request_body = CreatePostRequest,
    responses(
        (status = 200, description = "Post created successfully", body = SinglePostResponse),
        (status = 400, description = "Bad request", body = AppError),
        (status = 500, description = "Internal server error", body = AppError),
    ),
    tag= "Posts",
)]
pub async fn create(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.create_post(payload).await?;
    let response = SinglePostResponse { data: post };
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/posts/{id}",
    request_body = UpdatePostRequest,
    responses(
        (status = 200, description = "Post updated successfully", body = SinglePostResponse),
        (status = 400, description = "Bad request", body = AppError),
        (status = 401, description = "Unauthorized", body = AppError),
        (status =404, description = "Post not found", body = AppError),
    )
)]
pub async fn update(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.update_post(id, payload).await?;
    let response = SinglePostResponse { data: post };
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/posts",
    tag = "Posts",
    responses(
        (status = 200, description = "List of posts", body = ListPostResponse),
        (status = 404, description = "No posts found", body = AppError),
    )
)]
pub async fn list(
    State(state): State<Arc<ApplicationState>>,
) -> Result<Json<ListPostResponse>, AppError> {
    let posts = state.post_service.get_all_posts().await?;
    let response = ListPostResponse { data: posts };
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/posts/{id}",
    responses(
        (status = 200, description = "Post found", body = SinglePostResponse),
        (status = 404, description = "Post not found", body = AppError),
    )
)]
pub async fn get(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.get_post_by_id(id).await;
    match post {
        Ok(post) => {
            let response = SinglePostResponse { data: post };
            Ok(Json(response))
        }
        Err(e) => Err(AppError::from((StatusCode::NOT_FOUND, e))),
    }
}

#[utoipa::path(
    get,
    path = "/posts/slug/{name}",
    responses(
        (status = 200, description = "Post found", body = SinglePostResponse),
        (status = 404, description = "Post not found", body = AppError),
    )
)]
pub async fn get_by_slug(
    State(state): State<Arc<ApplicationState>>,
    Path(name): Path<String>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.get_post_by_slug(&name).await;
    match post {
        Ok(post) => {
            let response = SinglePostResponse { data: post };
            Ok(Json(response))
        }
        Err(e) => Err(AppError::from((StatusCode::NOT_FOUND, e))),
    }
}

#[utoipa::path(
    delete,
    path = "/posts/{id}",
    responses(
        (status = 200, description = "Post deleted successfully"),
        (status = 404, description = "Post not found", body = AppError),
    )
)]
pub async fn delete(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    state.post_service.delete_post(id).await?;
    Ok(())
}
