use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;

use crate::{
    apperr::AppError,
    model::Post,
    services::post::{CreatePostRequest, PostService, UpdatePostRequest},
    state::ApplicationState,
};

#[derive(Serialize)]
pub struct SinglePostResponse {
    pub data: Post,
}

#[derive(Serialize)]
pub struct ListPostResponse {
    pub data: Vec<Post>,
}

pub async fn create(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.create_post(payload).await?;
    let response = SinglePostResponse { data: post };
    Ok(Json(response))
}

pub async fn update(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.update_post(id, payload).await?;
    let response = SinglePostResponse { data: post };
    Ok(Json(response))
}

pub async fn list(
    State(state): State<Arc<ApplicationState>>,
) -> Result<Json<ListPostResponse>, AppError> {
    let posts = state.post_service.get_all_posts().await?;
    let response = ListPostResponse { data: posts };
    Ok(Json(response))
}

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

pub async fn delete(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    state.post_service.delete_post(id).await?;
    Ok(())
}
