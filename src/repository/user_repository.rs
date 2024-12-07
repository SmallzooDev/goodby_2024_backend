use crate::config::database::{Database, DatabaseTrait};
use crate::entity::user::User;
use crate::dto::user_dto::{UserRegisterDto, UserTicketInfo};
use async_trait::async_trait;
use sqlx::Error;
use std::sync::Arc;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn find_by_name(&self, name: String) -> Option<User>;
    async fn find(&self, id: i32) -> Result<User, Error>;
    async fn find_all(&self) -> Result<Vec<User>, Error>;
    async fn create(&self, payload: UserRegisterDto) -> Result<User, Error>;
    async fn find_team(&self, user_id: i32) -> Result<Option<(i32, String)>, Error>;
    async fn find_tickets(&self, user_id: i32) -> Result<Vec<UserTicketInfo>, Error>;
}

#[derive(Clone)]
pub struct UserRepository {
    db_conn: Arc<Database>,
}

impl UserRepository {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn find_by_name(&self, name: String) -> Option<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, name, phone_number, role, department_name
            FROM users
            WHERE name = $1
            "#,
            name
        )
        .fetch_optional(self.db_conn.get_pool())
        .await
        .ok()?
    }

    async fn find(&self, id: i32) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, phone_number, role, department_name
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(user)
    }

    async fn find_all(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as!(
            User,
            "SELECT id, name, phone_number, role, department_name FROM users"
        )
        .fetch_all(self.db_conn.get_pool())
        .await?;

        Ok(users)
    }

    async fn create(&self, payload: UserRegisterDto) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, phone_number, department_name)
            VALUES ($1, $2, 'purple')
            RETURNING id, name, phone_number, role, department_name
            "#,
            payload.name,
            payload.phone_number,
        )
        .fetch_one(self.db_conn.get_pool())
        .await
    }

    async fn find_team(&self, user_id: i32) -> Result<Option<(i32, String)>, Error> {
        let result = sqlx::query!(
            r#"
            SELECT t.id as team_id, t.team_name
            FROM team t
            JOIN users u ON u.team_id = t.id
            WHERE u.id = $1
            "#,
            user_id
        )
        .fetch_optional(self.db_conn.get_pool())
        .await?;

        Ok(result.map(|r| (r.team_id, r.team_name)))
    }

    async fn find_tickets(&self, user_id: i32) -> Result<Vec<UserTicketInfo>, Error> {
        let tickets = sqlx::query!(
            r#"
            SELECT ticket_number, available
            FROM user_tickets
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(self.db_conn.get_pool())
        .await?;

        Ok(tickets
            .into_iter()
            .map(|t| UserTicketInfo {
                ticket_number: t.ticket_number,
                available: t.available,
            })
            .collect())
    }
}
