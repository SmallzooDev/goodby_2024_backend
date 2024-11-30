use crate::config::database::Database;
use crate::dto::team_request_dto::{TeamAssignRequestDto, TeamCreateRequestDto, TeamUserDto};
use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::repository::team_repository::{TeamRepository, TeamRepositoryTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct TeamService {
    team_repo: TeamRepository,
    db_conn: Arc<Database>,
}

impl TeamService {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            team_repo: TeamRepository::new(db_conn),
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn create_team(&self, payload: TeamCreateRequestDto) -> Result<i32, ApiError> {
        let team_id = self
            .team_repo
            .create_team(payload.team_name)
            .await
            .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

        Ok(team_id)
    }

    pub async fn assign_team(&self, payload: TeamAssignRequestDto) -> Result<(), ApiError> {
        self.team_repo
            .assign_team(payload.users_id, payload.team_id)
            .await
            .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

        Ok(())
    }

    pub async fn get_team_users(&self) -> Result<Vec<TeamUserDto>, ApiError> {
        let team_users = self
            .team_repo
            .get_team_users()
            .await
            .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

        Ok(team_users)
    }
} 