use crate::config::database::{Database, DatabaseTrait};
use crate::entity::user::User;
use async_trait::async_trait;
use sqlx;
use sqlx::{Error, Postgres};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserRepository {
    pub(crate) db_conn: Arc<Database>,
}

#[async_trait]
pub trait UserRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;
    async fn find_by_name(&self, name: String) -> Option<User>;
    async fn find(&self, id: i64) -> Result<User, Error>;
    async fn find_all(&self) -> Result<Vec<User>, Error>;
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    async fn find_by_name(&self, name: String) -> Option<User> {
        let user = sqlx::query_as::<Postgres, User>("SELECT * FROM users WHERE name = $1")
            .bind(name)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None);
        user
    }

    async fn find(&self, id: i64) -> Result<User, Error> {
        let user = sqlx::query_as::<Postgres, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(self.db_conn.get_pool())
            .await;
        user
    }

    async fn find_all(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as::<Postgres, User>("SELECT * FROM users")
            .fetch_all(self.db_conn.get_pool())
            .await?;
        Ok(users)
    }
}