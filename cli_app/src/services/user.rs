use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

use crate::api::request::{CreateUserRequest, UpdateUserRequest};
use crate::api::response::{ListUserResponse, SingleUserResponse};
use crate::{
    model::{User, UserStatus},
    utils::password,
};

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

pub struct PgSqlUserService {
    pub pool: Pool<Postgres>,
}

impl PgSqlUserService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
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
            password: password::encrypt_password(&request.password)?,
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
        let user = data.items.get_mut(&id).unwrap();

        let last_login = user.last_login.clone();

        user.username = request.username;
        user.password = password::encrypt_password(&request.password)?;
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

impl UserService for PgSqlUserService {
    async fn get_all_users(&self) -> anyhow::Result<Vec<User>> {
        let res = sqlx::query!(
            r#"
            SELECT id, username, password, status, created, updated, last_login
            FROM users
            "#
        );
        res.fetch_all(&self.pool)
            .await
            .map(|rows| {
                rows.into_iter()
                    .map(|row| User {
                        id: row.id as i64,
                        username: row.username,
                        password: row.password,
                        status: UserStatus::from(row.status),
                        created: row.created.unwrap_or_default(),
                        updated: row.updated.unwrap_or_default(),
                        last_login: row.last_login,
                    })
                    .collect()
            })
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User> {
        let res = sqlx::query!(
            r#"
            SELECT id, username, password, status, created, updated, last_login
            FROM users
            WHERE id = $1
            "#,
            id
        );
        res.fetch_one(&self.pool)
            .await
            .map(|row| User {
                id: row.id as i64,
                username: row.username,
                password: row.password,
                status: UserStatus::from(row.status),
                created: row.created.unwrap_or_default(),
                updated: row.updated.unwrap_or_default(),
                last_login: row.last_login,
            })
            .map_err(|e| anyhow::anyhow!(e).context(format!("Failed to get user by id:{}", id)))
    }

    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<User> {
        let res = sqlx::query!(
            r#"
            SELECT id, username, password, status, created, updated, last_login
            FROM users
            WHERE username = $1
            "#,
            username
        );
        res.fetch_one(&self.pool)
            .await
            .map(|row| User {
                id: row.id as i64,
                username: row.username,
                password: row.password,
                status: UserStatus::from(row.status),
                created: row.created.unwrap_or_default(),
                updated: row.updated.unwrap_or_default(),
                last_login: row.last_login,
            })
            .map_err(|e| {
                anyhow::anyhow!(e).context(format!("Failed to get user by username:{}", username))
            })
    }

    async fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<User> {
        let query = sqlx::query!(
            r#"
                insert into users(username,password,status,created,updated,last_login) 
                values($1,$2,$3,Now(),Now(),null)
                returning id
            "#,
            request.username,
            password::encrypt_password(&request.password)?,
            i32::from(request.status)
        );

        let res = query.fetch_one(&self.pool).await?;
        let id: i64 = res.id as i64;
        let user = self.get_user_by_id(id).await?;
        Ok(user)
    }

    async fn update_user(&self, id: i64, request: UpdateUserRequest) -> anyhow::Result<User> {
        match self.get_user_by_id(id).await {
            Ok(user) => {
                let query = sqlx::query!(
                    r#"
                        update users
                        set username = $1, password = $2, status = $3, updated = Now(),last_login=$4 
                        where id = $5
                    "#,
                    request.username,
                    password::encrypt_password(&request.password)?,
                    i32::from(request.status),
                    user.updated,
                    id
                );
                query.execute(&self.pool).await?;
                let user = self.get_user_by_id(id).await?;
                Ok(user)
            }
            Err(_) => anyhow::bail!("User not found,id = {}", id),
        }
    }

    async fn delete_user(&self, id: i64) -> anyhow::Result<()> {
        let query = sqlx::query!(
            r#"
                delete from users
                where id = $1
            "#,
            id
        );
        query.execute(&self.pool).await?;
        Ok(())
    }
}
