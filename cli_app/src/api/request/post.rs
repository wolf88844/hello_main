use crate::model::PostStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreatePostRequest {
    pub author_id: i64,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: PostStatus,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdatePostRequest {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: PostStatus,
}
