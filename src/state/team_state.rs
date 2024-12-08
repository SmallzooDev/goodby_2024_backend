use crate::config::database::Database;
use crate::repository::team_repository::TeamRepository;
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
        let team_repo = TeamRepository::new(Arc::clone(db_conn));
        Self {
            team_service: TeamService::new(team_repo.clone()),
            team_repo,
        }
    }
} 