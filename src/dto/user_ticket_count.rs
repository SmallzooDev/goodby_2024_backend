use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserTicketCount {
    pub user_id: i32,
    pub name: String,
    pub ticket_count: Option<i64>,
}
