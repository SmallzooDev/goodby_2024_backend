use crate::config::database::{Database, DatabaseTrait};
use crate::entity::user::User;
use async_trait::async_trait;
use sqlx::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserRepository {
    pub(crate) db_conn: Arc<Database>,
}

#[async_trait]
pub trait UserRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;
    async fn find_by_name(&self, name: String) -> Option<User>;
    #[allow(dead_code)]
    async fn find(&self, id: i32) -> Result<User, Error>;
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
        sqlx::query_as!(
            User,
            "SELECT id, name, phone_number, role FROM users WHERE name = $1",
            name
        )
        .fetch_optional(self.db_conn.get_pool())
        .await
        .unwrap_or(None)
    }

    async fn find(&self, id: i32) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            "SELECT id, name, phone_number, role FROM users WHERE id = $1",
            id
        )
        .fetch_one(self.db_conn.get_pool())
        .await
    }

    async fn find_all(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as!(User, "SELECT id, name, phone_number, role FROM users")
            .fetch_all(self.db_conn.get_pool())
            .await?;

        Ok(users)
    }
}
