use std::sync::Arc;

use axum::{extract::{Path, State}, Json};
use serde::Serialize;

use crate::{apperr::AppError, model::User, services::user::{CreateUserRequest, UpdateUserRequest, UserService}, state::ApplicationState};

#[derive(Serialize)]
pub struct ListUserResponse {
    pub data: Vec<User>,
}

#[derive(Serialize)]
pub struct SingleUserResponse {
    pub data: User,
}

pub async fn create(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<CreateUserRequest>,
)->Result<Json<SingleUserResponse>, AppError>{
    let user = state.user_service.create_user(payload).await?;
    let response = SingleUserResponse { data: user };
    Ok(Json(response))
}

pub async fn list(
    State(state): State<Arc<ApplicationState>>,
)->Result<Json<ListUserResponse>, AppError>{
    let users = state.user_service.get_all_users().await?;
    let response = ListUserResponse { data: users };
    Ok(Json(response))
}

pub async fn get(
    State(state): State<Arc<ApplicationState>>,
    Path(id):Path<i64>,
)->Result<Json<SingleUserResponse>, AppError>{
    let user = state.user_service.get_user_by_id(id).await;
    match user{
        Ok(user) => {
            let response = SingleUserResponse { data: user };
            Ok(Json(response))
        }
        Err(e) => Err(AppError::from((axum::http::StatusCode::NOT_FOUND, e))),
    }   
}

pub async fn get_by_username(
    State(state): State<Arc<ApplicationState>>,
    Path(username):Path<String>,
)->Result<Json<SingleUserResponse>, AppError>{
    let user = state.user_service.get_user_by_username(&username).await;
    match user{
        Ok(user) => {
            let response = SingleUserResponse { data: user };
            Ok(Json(response)
            )
        }
        Err(e) => Err(AppError::from((axum::http::StatusCode::NOT_FOUND, e))),
    }
}

pub async fn update(
    State(state): State<Arc<ApplicationState>>,
    Path(id):Path<i64>,
    Json(payload): Json<UpdateUserRequest>,
)->Result<Json<SingleUserResponse>, AppError>{
    let user = state.user_service.update_user(id, payload).await?;
    let response = SingleUserResponse { data: user };
    Ok(Json(response))
}

pub async fn delete(
    State(state): State<Arc<ApplicationState>>,
    Path(id):Path<i64>,
)->Result<(), AppError>{
    state.user_service.delete_user(id).await?;
    Ok(())
}