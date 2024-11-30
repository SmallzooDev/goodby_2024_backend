use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TeamAssignRequestDto {
    pub users_id: Vec<i32>,
    pub team_id: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TeamCreateRequestDto {
    pub team_name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TeamUserDto {
    pub team_id: i32,
    pub team_name: String,
    pub users: Vec<TeamUserInfoDto>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TeamUserInfoDto {
    pub user_id: i32,
    pub name: String,
    pub ticket_count: i64,
} 