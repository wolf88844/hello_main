use crate::model::UserStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub status: UserStatus,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub status: UserStatus,
}
