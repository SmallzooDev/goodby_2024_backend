use crate::config::database::{Database, DatabaseTrait};
use crate::dto::user_dto::{UserReadDto, UserRegisterDto, UserMeDto, UserTeamDto};
use crate::entity::user::User;
use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::user_error::UserError;
use crate::repository::user_repository::{UserRepository, UserRepositoryTrait};
use sqlx::Error as SqlxError;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    db_conn: Arc<Database>,
}

impl UserService {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_repo: UserRepository::new(db_conn),
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn create_user(&self, payload: UserRegisterDto) -> Result<UserReadDto, ApiError> {
        return match self.user_repo.find_by_name(payload.name.to_owned()).await {
            Some(_) => Err(UserError::UserAlreadyExists)?,
            None => {
                let user = self.add_user(payload).await;

                return match user {
                    Ok(user) => Ok(UserReadDto::from(user)),
                    Err(e) => match e {
                        SqlxError::Database(e) => match e.code() {
                            Some(code) => {
                                if code == "23000" {
                                    Err(DbError::UniqueConstraintViolation(e.to_string()))?
                                } else {
                                    Err(DbError::SomethingWentWrong(e.to_string()))?
                                }
                            }
                            _ => Err(DbError::SomethingWentWrong(e.to_string()))?,
                        },
                        _ => Err(DbError::SomethingWentWrong(e.to_string()))?,
                    },
                };
            }
        };
    }

    async fn add_user(&self, payload: UserRegisterDto) -> Result<User, SqlxError> {
        let user = sqlx::query_as!(
            User,
            r#"
        INSERT INTO users (name, phone_number)
        VALUES ($1, $2)
        RETURNING id, name, phone_number, role
        "#,
            payload.name,
            payload.phone_number,
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(user)
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

    pub async fn get_user_details(&self, user_id: i32) -> Result<UserMeDto, ApiError> {
        let user = self.user_repo.find(user_id).await
            .map_err(|e| DbError::SomethingWentWrong(e.to_string()))?;

        let team = sqlx::query!(
            r#"
            SELECT t.id as team_id, t.team_name
            FROM team t
            JOIN users u ON u.team_id = t.id
            WHERE u.id = $1
            "#,
            user_id
        )
        .fetch_optional(self.db_conn.get_pool())
        .await
        .map_err(|e| DbError::SomethingWentWrong(e.to_string()))?;

        let tickets = sqlx::query!(
            r#"
            SELECT ticket_number
            FROM user_tickets
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(self.db_conn.get_pool())
        .await
        .map_err(|e| DbError::SomethingWentWrong(e.to_string()))?;

        let ticket_count = tickets.len() as i64;
        let ticket_numbers = tickets.into_iter()
            .map(|t| t.ticket_number)
            .collect();

        Ok(UserMeDto {
            id: user.id,
            name: user.name,
            role: user.role,
            phone_number: user.phone_number,
            team: team.map(|t| UserTeamDto {
                team_id: t.team_id,
                team_name: t.team_name,
            }),
            ticket_count,
            tickets: ticket_numbers,
        })
    }
}

