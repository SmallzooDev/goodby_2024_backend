use crate::dto::user_dto::{UserMeDto, UserReadDto, UserRegisterDto, UserTeamDto};
use crate::entity::user::User;
use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::user_error::UserError;
use crate::repository::user_repository::UserRepositoryTrait;
use sqlx::Error as SqlxError;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<dyn UserRepositoryTrait>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepositoryTrait>) -> Self {
        Self { user_repo }
    }

    pub async fn create_user(&self, payload: UserRegisterDto) -> Result<UserReadDto, ApiError> {
        if let Some(_) = self.user_repo.find_by_name(payload.name.clone()).await {
            return Err(UserError::UserAlreadyExists)?;
        }

        match self.user_repo.create(payload).await {
            Ok(user) => Ok(UserReadDto::from(user)),
            Err(e) => match e {
                SqlxError::Database(e) => match e.code() {
                    Some(code) if code == "23000" => {
                        Err(DbError::UniqueConstraintViolation(e.to_string()))?
                    }
                    _ => Err(DbError::SomethingWentWrong(e.to_string()))?,
                },
                _ => Err(DbError::SomethingWentWrong(e.to_string()))?,
            },
        }
    }

    pub async fn get_user_details(&self, user_id: i32) -> Result<UserMeDto, ApiError> {
        let user = self.user_repo.find(user_id).await
            .map_err(|e| DbError::SomethingWentWrong(e.to_string()))?;

        let team = self.user_repo.find_team(user_id).await
            .map_err(|e| DbError::SomethingWentWrong(e.to_string()))?;

        let tickets = self.user_repo.find_tickets(user_id).await
            .map_err(|e| DbError::SomethingWentWrong(e.to_string()))?;

        Ok(UserMeDto {
            id: user.id,
            name: user.name,
            role: user.role,
            phone_number: user.phone_number,
            department_name: user.department_name,
            team: team.map(|(team_id, team_name)| UserTeamDto {
                team_id,
                team_name,
            }),
            ticket_count: tickets.len() as i64,
            tickets,
        })
    }

    pub fn verify_phone_number(&self, user: &User, phone_number: &str) -> bool {
        user.phone_number == phone_number
    }
    pub async fn get_all_users(&self) -> Result<Vec<UserReadDto>, DbError> {
        let users = self
            .user_repo
            .find_all()
            .await
            .map_err(|e| DbError::SomethingWentWrong(e.to_string()))?;

        let user_dtos = users.into_iter().map(UserReadDto::from).collect();
        Ok(user_dtos)
    }

    pub async fn find_by_name(&self, name: String) -> Option<User> {
        self.user_repo.find_by_name(name).await
    }
}

