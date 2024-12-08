use crate::config::database::DatabaseTrait;
use crate::dto::prize_draw_dto::{DrawPrizeRequestDto, PrizeDrawDto};
use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::repository::prize_draw_repository::{PrizeDrawRepository, PrizeDrawRepositoryTrait};
use crate::repository::prize_repository::{PrizeRepository, PrizeRepositoryTrait};
use crate::repository::user_ticket_repository::{UserTicketRepository, UserTicketRepositoryTrait};
use std::iter::IntoIterator;

#[derive(Clone)]
pub struct PrizeDrawService {
    prize_draw_repo: PrizeDrawRepository,
    prize_repo: PrizeRepository,
    user_ticket_repo: UserTicketRepository,
}

impl PrizeDrawService {
    pub fn new(
        prize_draw_repo: PrizeDrawRepository,
        prize_repo: PrizeRepository,
        user_ticket_repo: UserTicketRepository,
    ) -> Self {
        Self {
            prize_draw_repo,
            prize_repo,
            user_ticket_repo,
        }
    }

    pub async fn draw_prize(&self, payload: DrawPrizeRequestDto) -> Result<Vec<PrizeDrawDto>, ApiError> {
        // 트랜잭션 시작
        let mut tx = self
            .prize_draw_repo
            .db_conn
            .get_pool()
            .begin()
            .await
            .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

        // 상품 정보 조회
        let prize = self
            .prize_repo
            .find_by_id_in_tx(&mut tx, payload.prize_id)
            .await
            .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

        // 당첨 가능한 티켓 목록 조회
        let available_tickets = self
            .user_ticket_repo
            .get_available_tickets_for_draw(&mut tx, payload.count as i64)
            .await
            .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

        let mut results = Vec::new();
        let ticket_numbers: Vec<String> = available_tickets
            .iter()
            .map(|ticket| ticket.ticket_number.clone())
            .collect();

        // 티켓 사용 처리
        self.user_ticket_repo
            .mark_tickets_as_used(&mut tx, &ticket_numbers)
            .await
            .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

        // 당첨 결과 저장
        for ticket in available_tickets {
            let draw = self
                .prize_draw_repo
                .create_draw_in_tx(
                    &mut tx,
                    prize.id,
                    prize.name.clone(),
                    ticket.user_id,
                    ticket.user_name,
                    ticket.department_name,
                    ticket.ticket_number,
                )
                .await
                .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

            results.push(PrizeDrawDto {
                id: draw.id,
                prize_name: draw.prize_name,
                user_name: draw.user_name,
                department_name: draw.department_name,
                ticket_number: draw.ticket_number,
                created_at: draw.created_at.to_string(),
            });
        }

        // 트랜잭션 커밋
        tx.commit()
            .await
            .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

        Ok(results)
    }

    pub async fn get_all_draws(&self) -> Result<Vec<PrizeDrawDto>, ApiError> {
        let draws = self
            .prize_draw_repo
            .find_all()
            .await
            .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

        Ok(draws
            .into_iter()
            .map(|d| PrizeDrawDto {
                id: d.id,
                prize_name: d.prize_name,
                user_name: d.user_name,
                department_name: d.department_name,
                ticket_number: d.ticket_number,
                created_at: d.created_at.to_string(),
            })
            .collect())
    }
} 