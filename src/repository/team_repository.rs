use crate::config::database::{Database, DatabaseTrait};
use crate::dto::team_request_dto::{TeamUserDto, TeamUserInfoDto};
use async_trait::async_trait;
use sqlx::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct TeamRepository {
    db_conn: Arc<Database>,
}

#[async_trait]
pub trait TeamRepositoryTrait: Send + Sync {
    async fn create_team(&self, team_name: String) -> Result<i32, Error>;
    async fn assign_team(&self, user_ids: Vec<i32>, team_id: i32) -> Result<(), Error>;
    async fn get_team_users(&self) -> Result<Vec<TeamUserDto>, Error>;
}

impl TeamRepository {
    pub fn new(db_conn: Arc<Database>) -> Self {
        Self {
            db_conn
        }
    }
}

#[async_trait]
impl TeamRepositoryTrait for TeamRepository {
    async fn create_team(&self, team_name: String) -> Result<i32, Error> {
        let result = sqlx::query_scalar!(
            r#"
            INSERT INTO team (team_name)
            VALUES ($1)
            RETURNING id
            "#,
            team_name
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(result)
    }

    async fn assign_team(&self, user_ids: Vec<i32>, team_id: i32) -> Result<(), Error> {
        for user_id in user_ids {
            sqlx::query!(
                r#"
                UPDATE users
                SET team_id = $1
                WHERE id = $2
                "#,
                team_id,
                user_id
            )
            .execute(self.db_conn.get_pool())
            .await?;
        }

        Ok(())
    }

    async fn get_team_users(&self) -> Result<Vec<TeamUserDto>, Error> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                t.id as "team_id!",
                t.team_name as "team_name!",
                COALESCE(u.id, 0) as "user_id!",
                COALESCE(u.name, '') as "name!",
                COALESCE(COUNT(ut.id), 0)::bigint as "ticket_count!"
            FROM team t
            LEFT JOIN users u ON u.team_id = t.id
            LEFT JOIN user_tickets ut ON ut.user_id = u.id
            GROUP BY t.id, t.team_name, u.id, u.name
            ORDER BY t.id, u.name
            "#
        )
        .fetch_all(self.db_conn.get_pool())
        .await?;

        let mut team_users: Vec<TeamUserDto> = Vec::new();
        let mut current_team: Option<TeamUserDto> = None;

        for row in rows {
            if current_team.is_none() || current_team.as_ref().unwrap().team_id != row.team_id {
                if let Some(team) = current_team {
                    team_users.push(team);
                }
                current_team = Some(TeamUserDto {
                    team_id: row.team_id,
                    team_name: row.team_name,
                    users: Vec::new(),
                });
            }

            if let Some(team) = current_team.as_mut() {
                if row.user_id != 0 {
                    team.users.push(TeamUserInfoDto {
                        user_id: row.user_id,
                        name: row.name,
                        ticket_count: row.ticket_count,
                    });
                }
            }
        }

        if let Some(team) = current_team {
            team_users.push(team);
        }

        Ok(team_users)
    }
} 