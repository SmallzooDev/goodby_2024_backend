use crate::parameter;
use async_trait::async_trait;
use sqlx::{Error, PgPool};

pub struct Database {
    pool: PgPool,
}

#[async_trait]
pub trait DatabaseTrait {
    async fn init() -> Result<Self, Error>
    where
        Self: Sized;
    fn get_pool(&self) -> &PgPool;
}

#[async_trait]
impl DatabaseTrait for Database {
    async fn init() -> Result<Self, Error> {
        let database_url = parameter::get("DATABASE_URL");
        let pool = PgPool::connect(&database_url).await?;
        Ok(Self { pool })
    }

    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}