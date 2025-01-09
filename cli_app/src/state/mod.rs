use std::sync::Arc;

use crate::{
    Settings,
    services::{post::PgSqlPostService, user::PgSqlUserService},
};
use anyhow::Ok;
use arc_swap::ArcSwap;
use sqlx::PgPool;

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
    pub user_service: Arc<PgSqlUserService>,
    pub post_service: Arc<PgSqlPostService>,
}

impl ApplicationState {
    pub fn new(settings: &Settings, pool: PgPool) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
            user_service: Arc::new(PgSqlUserService::new(pool.clone())),
            post_service: Arc::new(PgSqlPostService::new(pool.clone())),
        })
    }
}
