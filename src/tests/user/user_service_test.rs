#[cfg(test)]
mod tests {
    use crate::entity::user::User;
    use crate::dto::user_dto::UserRegisterDto;
    use crate::error::{api_error::ApiError, user_error::UserError};
    use mockall::mock;
    use std::sync::Arc;
    use sqlx::Error as SqlxError;
    use async_trait::async_trait;
    use crate::dto::user_dto::UserTicketInfo;
    use crate::repository::user_repository::UserRepositoryTrait;
    use crate::service::user_service::UserService;

    mock! {
        pub UserRepo {}

        #[async_trait]
        impl UserRepositoryTrait for UserRepo {
            async fn find_by_name(&self, name: String) -> Option<User>;
            async fn find(&self, id: i32) -> Result<User, SqlxError>;
            async fn find_all(&self) -> Result<Vec<User>, SqlxError>;
            async fn create(&self, payload: UserRegisterDto) -> Result<User, SqlxError>;
            async fn find_team(&self, user_id: i32) -> Result<Option<(i32, String)>, SqlxError>;
            async fn find_tickets(&self, user_id: i32) -> Result<Vec<UserTicketInfo>, SqlxError>;
        }
    }

    #[tokio::test]
    async fn test_create_user_when_user_not_exists() {
        let mut mock_repo = MockUserRepo::new();
        
        mock_repo
            .expect_find_by_name()
            .with(mockall::predicate::eq("test".to_string()))
            .times(1)
            .return_once(|_| None);

        mock_repo
            .expect_create()
            .with(mockall::predicate::eq(UserRegisterDto {
                name: "test".to_string(),
                phone_number: "1234".to_string(),
            }))
            .times(1)
            .return_once(|_| Ok(User {
                id: 1,
                name: "test".to_string(),
                phone_number: "1234".to_string(),
                role: "user".to_string(),
                department_name: "purple".to_string(),
            }));

        let service = UserService::new(Arc::new(mock_repo));
        let result = service
            .create_user(UserRegisterDto {
                name: "test".to_string(),
                phone_number: "1234".to_string(),
            })
            .await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "test");
    }

    #[tokio::test]
    async fn test_create_user_when_user_exists() {
        let mut mock_repo = MockUserRepo::new();
        
        mock_repo
            .expect_find_by_name()
            .with(mockall::predicate::eq("test".to_string()))
            .times(1)
            .return_once(|_| Some(User {
                id: 1,
                name: "test".to_string(),
                phone_number: "1234".to_string(),
                role: "user".to_string(),
                department_name: "purple".to_string(),
            }));

        let service = UserService::new(Arc::new(mock_repo));
        let result = service
            .create_user(UserRegisterDto {
                name: "test".to_string(),
                phone_number: "1234".to_string(),
            })
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::User(UserError::UserAlreadyExists)));
    }
} 



