use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

use crate::model::{Post, PostStatus};

pub struct InMemoryPostStore {
    pub counter: i64,
    pub items: HashMap<i64, Post>,
}

pub struct InMemoryPostService {
    data: Mutex<InMemoryPostStore>,
}

impl Default for InMemoryPostService {
    fn default() -> Self {
        Self {
            data: Mutex::new(InMemoryPostStore {
                counter: 0,
                items: HashMap::new(),
            }),
        }
    }
}

pub struct PgSqlPostService {
    pub pool: Pool<Postgres>,
}

impl PgSqlPostService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub author_id: i64,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: PostStatus,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePostRequest {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: PostStatus,
}

#[allow(async_fn_in_trait)]
pub trait PostService {
    async fn get_all_posts(&self) -> anyhow::Result<Vec<Post>>;
    async fn get_post_by_id(&self, id: i64) -> anyhow::Result<Post>;
    async fn get_post_by_slug(&self, name: &str) -> anyhow::Result<Post>;
    async fn create_post(&self, req: CreatePostRequest) -> anyhow::Result<Post>;
    async fn update_post(&self, id: i64, req: UpdatePostRequest) -> anyhow::Result<Post>;
    async fn delete_post(&self, id: i64) -> anyhow::Result<()>;
}

impl PostService for InMemoryPostService {
    async fn get_all_posts(&self) -> anyhow::Result<Vec<Post>> {
        let data = self.data.lock().await;
        Ok(data.items.values().map(|post| (*post).clone()).collect())
    }

    async fn get_post_by_id(&self, id: i64) -> anyhow::Result<Post> {
        let data = self.data.lock().await;
        match data.items.get(&id) {
            Some(post) => Ok(post.clone()),
            None => Err(anyhow::anyhow!("Post not found: {}", id)),
        }
    }

    async fn get_post_by_slug(&self, name: &str) -> anyhow::Result<Post> {
        let data = self.data.lock().await;
        for (_id, post) in data.items.iter() {
            if post.slug == name {
                return Ok(post.clone());
            }
        }
        anyhow::bail!("Post not found: {}", name);
    }

    async fn create_post(&self, req: CreatePostRequest) -> anyhow::Result<Post> {
        let mut data = self.data.lock().await;
        data.counter += 1;
        let ts = chrono::offset::Utc::now();
        let post = Post {
            id: data.counter,
            author_id: req.author_id,
            title: req.title,
            slug: req.slug,
            content: req.content,
            status: req.status,
            created: ts,
            updated: ts,
        };
        data.items.insert(post.id, post);

        match data.items.get(&data.counter) {
            None => {
                anyhow::bail!("Post not found: {}", data.counter);
            }
            Some(post) => Ok(post.clone()),
        }
    }

    async fn update_post(&self, id: i64, req: UpdatePostRequest) -> anyhow::Result<Post> {
        let mut data = self.data.lock().await;
        let post = data.items.get_mut(&id).unwrap();
        post.slug = req.slug;
        post.title = req.title;
        post.content = req.content;
        post.status = req.status;

        match data.items.get(&data.counter) {
            None => {
                anyhow::bail!("Post not found: {}", data.counter);
            }
            Some(post) => Ok(post.clone()),
        }
    }

    async fn delete_post(&self, id: i64) -> anyhow::Result<()> {
        let mut data = self.data.lock().await;
        match data.items.remove(&id) {
            Some(_) => Ok(()),
            None => Err(anyhow::anyhow!("Post not found: {}", id)),
        }
    }
}

impl PostService for PgSqlPostService {
    async fn get_all_posts(&self) -> anyhow::Result<Vec<Post>> {
        let res = sqlx::query!(
            r#"
            SELECT id, author_id, title, slug, content, status, created, updated
            FROM posts
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        let list = res
            .into_iter()
            .map(|row| Post {
                id: row.id,
                author_id: row.author_id,
                title: row.title,
                slug: row.slug,
                content: row.content,
                status: PostStatus::from(row.status),
                created: row.created.unwrap_or_default(),
                updated: row.updated.unwrap_or_default(),
            })
            .collect();

        Ok(list)
    }

    async fn get_post_by_id(&self, id: i64) -> anyhow::Result<Post> {
        let res = sqlx::query!(
            r#"
            SELECT id, author_id,title, slug, content, status, created, updated
            FROM posts
            WHERE id = $1
            "#,
            id
        );
        res.fetch_one(&self.pool)
            .await
            .map(|row| Post {
                id: row.id,
                author_id: row.author_id,
                title: row.title,
                slug: row.slug,
                content: row.content,
                status: PostStatus::from(row.status),
                created: row.created.unwrap_or_default(),
                updated: row.updated.unwrap_or_default(),
            })
            .map_err(|e| anyhow::anyhow!(e).context(format!("Post not found: {}", id)))
    }

    async fn get_post_by_slug(&self, name: &str) -> anyhow::Result<Post> {
        let res = sqlx::query!(
            r#"
            SELECT id, author_id,title, slug, content, status, created, updated
            FROM posts
            WHERE slug = $1
            "#,
            name
        );
        res.fetch_one(&self.pool)
            .await
            .map(|row| Post {
                id: row.id,
                author_id: row.author_id,
                title: row.title,
                slug: row.slug,
                content: row.content,
                status: PostStatus::from(row.status),
                created: row.created.unwrap_or_default(),
                updated: row.updated.unwrap_or_default(),
            })
            .map_err(|e| anyhow::anyhow!(e).context(format!("Post not found: {}", name)))
    }

    async fn create_post(&self, req: CreatePostRequest) -> anyhow::Result<Post> {
        let res = sqlx::query!(
            r#"
            INSERT INTO posts (author_id, title, slug, content, status, created, updated)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())   
            RETURNING id
            "#,
            req.author_id,
            req.title,
            req.slug,
            req.content,
            i32::from(req.status),
        );
        let res = res.fetch_one(&self.pool).await?;
        let id = res.id;
        self.get_post_by_id(id).await
    }

    async fn update_post(&self, id: i64, req: UpdatePostRequest) -> anyhow::Result<Post> {
        match self.get_post_by_id(id).await {
            Ok(_post) => {
                let res = sqlx::query!(
                    r#"
                    UPDATE posts
                    SET author_id = $1, title = $2, slug = $3, content = $4, status = $5, updated = NOW()
                    WHERE id = $6
                    "#,
                    req.author_id,
                    req.title,
                    req.slug,
                    req.content,
                    i32::from(req.status),
                    id
                );
                res.execute(&self.pool).await?;
                self.get_post_by_id(id).await
            }
            Err(_) => {
                anyhow::bail!("Post not found: {}", id);
            }
        }
    }

    async fn delete_post(&self, id: i64) -> anyhow::Result<()> {
        let res = sqlx::query!(
            r#"
            DELETE FROM posts
            WHERE id = $1
            "#,
            id
        );
        res.execute(&self.pool).await?;
        Ok(())
    }
}
