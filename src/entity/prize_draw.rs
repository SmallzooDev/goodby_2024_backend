use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct PrizeDraw {
    pub id: i32,
    pub prize_id: i32,
    pub prize_name: String,
    pub user_id: i32,
    pub user_name: String,
    pub department_name: String,
    pub ticket_number: String,
    pub created_at: OffsetDateTime,
} 