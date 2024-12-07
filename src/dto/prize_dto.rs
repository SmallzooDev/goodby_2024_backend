use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct CreatePrizeDto {
    pub name: String,
    pub stock: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrizeDto {
    pub id: i32,
    pub name: String,
    pub stock: i32,
} 