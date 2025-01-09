use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::model::{Post, PostStatue};

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

#[derive(Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub author_id: i64,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub statue: PostStatue,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePostRequest {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub statue: PostStatue,
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
            statue: req.statue,
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
        let post = data
            .items
            .get_mut(&id)
            .unwrap();
        post.slug = req.slug;
        post.title = req.title;
        post.content = req.content;
        post.statue = req.statue;

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
