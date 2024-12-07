use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow)]
pub struct AvailableTicket {
    pub user_id: i32,
    pub user_name: String,
    pub department_name: String,
    pub ticket_number: String,
} 