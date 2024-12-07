use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserTicket {
    pub id: i32,
    pub user_id: i32,
    pub ticket_number: String,
    pub role: String,
    pub available: bool,
}
