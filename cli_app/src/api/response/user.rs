use crate::model::User;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ListUserResponse {
    pub data: Vec<User>,
}

#[derive(Serialize, ToSchema)]
pub struct SingleUserResponse {
    pub data: User,
}
