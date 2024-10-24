use crate::config::database::{Database, DatabaseTrait};
use crate::dto::user_dto::{UserReadDto, UserRegisterDto};
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
}

