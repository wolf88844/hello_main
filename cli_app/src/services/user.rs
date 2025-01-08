use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::model::{User, UserStatus};

pub struct InmemoryUserStore {
    pub counter: i64,
    pub items: HashMap<i64, User>,
}

pub struct InMemoryUserService {
    data: Mutex<InmemoryUserStore>,
}

impl Default for InMemoryUserService {
    fn default() -> Self {
        InMemoryUserService {
            data: Mutex::new(InmemoryUserStore {
                counter: 0,
                items: HashMap::new(),
            }),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub status: UserStatus,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub status: UserStatus,
}

#[allow(async_fn_in_trait)]
pub trait UserService {
    async fn get_all_users(&self) -> anyhow::Result<Vec<User>>;
    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User>;
    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<User>;

    async fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<User>;
    async fn update_user(&self, id: i64, request: UpdateUserRequest) -> anyhow::Result<User>;
    async fn delete_user(&self, id: i64) -> anyhow::Result<()>;
}

impl UserService for InMemoryUserService {
    async fn get_all_users(&self) -> anyhow::Result<Vec<User>> {
        let data = self.data.lock().await;
        Ok(data.items.values().map(|user| (*user).clone()).collect())
    }

    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User> {
        let data = self.data.lock().await;
        match data.items.get(&id) {
            None => Err(anyhow::anyhow!("User not found:{}", id)),
            Some(user) => Ok(user.clone()),
        }
    }

    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<User> {
        let data = self.data.lock().await;
        for (_id, user) in data.items.iter() {
            if user.username == username {
                return Ok(user.clone());
            }
        }
        anyhow::bail!("User not found:{}", username)
    }

    async fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<User> {
        let mut data = self.data.lock().await;
        data.counter += 1;
        let ts = chrono::offset::Utc::now();
        let user = User {
            id: data.counter,
            username: request.username,
            password: request.password,
            status: request.status,
            created: ts,
            updated: ts,
            last_login: None,
        };
        data.items.insert(user.id, user);

        match data.items.get(&data.counter) {
            None => anyhow::bail!("User not found:{}", data.counter),
            Some(user) => Ok(user.clone()),
        }
    }

    async fn update_user(&self, id: i64, request: UpdateUserRequest) -> anyhow::Result<User> {
        let mut data = self.data.lock().await;
        let user = data
            .items
            .get_mut(&id)
            .ok_or(anyhow::bail!("User not found:{}", request.id))?;

        let last_login = user.last_login.clone();

        user.username = request.username;
        user.password = request.password;
        user.status = request.status;
        user.updated = chrono::offset::Utc::now();
        user.last_login = last_login;

        match data.items.get(&data.counter) {
            None => anyhow::bail!("User not found:{}", request.id),
            Some(user) => Ok(user.clone()),
        }
    }

    async fn delete_user(&self, id: i64) -> anyhow::Result<()> {
        let mut data = self.data.lock().await;
        match data.items.remove(&id) {
            None => anyhow::bail!("User not found:{}", id),
            Some(_) => Ok(()),
        }
    }
}
