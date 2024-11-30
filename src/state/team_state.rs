use crate::config::database::Database;
use crate::repository::team_repository::{TeamRepository, TeamRepositoryTrait};
use crate::service::team_service::TeamService;
use std::sync::Arc;

#[derive(Clone)]
pub struct TeamState {
    pub team_service: TeamService,
    #[allow(dead_code)]
    pub team_repo: TeamRepository,
}

impl TeamState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            team_service: TeamService::new(db_conn),
            team_repo: TeamRepository::new(db_conn),
        }
    }
} 