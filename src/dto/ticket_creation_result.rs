use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct TicketCreationResult {
    pub user_id: i32,
    pub ticket_number: String,
    pub message: String,
    pub available: bool,
}