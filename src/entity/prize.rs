use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Prize {
    pub id: i32,
    pub name: String,
    pub stock: i32,
} 