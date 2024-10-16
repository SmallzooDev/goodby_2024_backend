use crate::entity::user::User;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UserLoginDto {
    pub name: String,
    pub phone_number: String,
}

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UserRegisterDto {
    pub name: String,
    pub phone_number: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserReadDto {
    pub id: i32,
    pub name: String,
    pub role: String,
}

impl UserReadDto {
    pub fn from(model: User) -> UserReadDto {
        Self {
            id: model.id,
            name: model.name,
            role: model.role,
        }
    }
}

impl std::fmt::Debug for UserLoginDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User").field("name", &self.name).finish()
    }
}

impl std::fmt::Debug for UserRegisterDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User").field("name", &self.name).finish()
    }
}
