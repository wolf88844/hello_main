use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum UserStatus {
    Active = 1,
    Blocked = 2,
}

impl From<i32> for UserStatus {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Active,
            2 => Self::Blocked,
            _ => Self::Active,
        }
    }
}

impl From<UserStatus> for i32 {
    fn from(value: UserStatus) -> Self {
        match value {
            UserStatus::Active => 1,
            UserStatus::Blocked => 2,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum PostStatue {
    Draft = 1,
    Published = 2,
}

impl From<i32> for PostStatue {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Draft,
            2 => Self::Published,
            _ => Self::Draft,
        }
    }
}

impl From<PostStatue> for i32 {
    fn from(value: PostStatue) -> Self {
        match value {
            PostStatue::Draft => 1,
            PostStatue::Published => 2,
        }
    }
}
#[derive(Clone, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub status: UserStatus,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Clone, Serialize)]
pub struct Post {
    pub id: i64,
    pub author_id: i64,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub statue: PostStatue,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
