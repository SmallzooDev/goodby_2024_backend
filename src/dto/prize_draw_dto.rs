use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct DrawPrizeRequestDto {
    pub prize_id: i32,
    pub count: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PrizeDrawDto {
    pub id: i32,
    pub prize_name: String,
    pub user_name: String,
    pub department_name: String,
    pub ticket_number: String,
    pub created_at: String,
} 