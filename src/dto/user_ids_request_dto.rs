use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserIdsRequestDto {
    pub users_id: Vec<i32>,
}
