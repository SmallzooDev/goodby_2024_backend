use crate::config::database::{Database, DatabaseTrait};
use crate::entity::user::User;
use async_trait::async_trait;
use sqlx;
use sqlx::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserRepository {
    pub(crate) db_conn: Arc<Database>,
}

#[async_trait]
pub trait UserRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;
    async fn find_by_name(&self, email: String) -> Option<User>;
    async fn find(&self, id: u64) -> Result<User, Error>;
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    async fn find_by_name(&self, name: String) -> Option<User> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = ?")
            .bind(name)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None);
        return user;
    }

    async fn find(&self, id: u64) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(self.db_conn.get_pool())
            .await;
        return user;
    }
}
