use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_field_unit::repository::game_field_unit_repository::GameFieldUnitRepository;
use crate::game_field_unit::repository::game_field_unit_repository_impl::GameFieldUnitRepositoryImpl;
use crate::game_field_unit_action_possibility_validator::service::game_field_unit_action_possibility_validator_service::GameFieldUnitActionValidatorService;
use crate::game_field_unit_action_possibility_validator::service::request::is_unit_basic_attack_possible_request::{IsUnitBasicAttackPossibleRequest};
use crate::game_field_unit_action_possibility_validator::service::response::is_unit_basic_attack_possible_response::{IsUnitBasicAttackPossibleResponse};
use crate::game_round::repository::game_round_repository_impl::GameRoundRepositoryImpl;

pub struct GameFieldUnitActionValidatorServiceImpl {
    game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
    game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
}

impl GameFieldUnitActionValidatorServiceImpl {
    pub fn new(game_round_repository: Arc<AsyncMutex<GameRoundRepositoryImpl>>,
               game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>) -> Self {

        GameFieldUnitActionValidatorServiceImpl {
            game_round_repository,
            game_field_unit_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameFieldUnitActionValidatorServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameFieldUnitActionValidatorServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameFieldUnitActionValidatorServiceImpl::new(
                            GameRoundRepositoryImpl::get_instance(),
                            GameFieldUnitRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn get_field_unit_turn_action(&self,
                                        account_unique_id: i32,
                                        field_unit_index: i32) -> Option<bool> {
        let mut game_field_unit_repository_guard =
            self.game_field_unit_repository.lock().await;

        game_field_unit_repository_guard
            .get_game_field_unit_map()
            .get_mut(&account_unique_id)
            .map(|mut field_unit| field_unit
                .check_turn_action_of_unit(field_unit_index as usize))
    }

    async fn get_field_unit_deployed_round(&self,
                                           account_unique_id: i32,
                                           field_unit_index: i32) -> Option<i32> {
        let mut game_field_unit_repository_guard =
            self.game_field_unit_repository.lock().await;

        game_field_unit_repository_guard
            .get_game_field_unit_map()
            .get_mut(&account_unique_id)
            .map(|mut field_unit| field_unit
                .get_unit_deployed_round(field_unit_index as usize))
    }

    async fn get_total_energy_count_of_field_unit(&self,
                                                  account_unique_id: i32,
                                                  field_unit_index: i32) -> Option<i32> {
        let mut game_field_unit_repository_guard =
            self.game_field_unit_repository.lock().await;

        game_field_unit_repository_guard
            .get_game_field_unit_map()
            .get_mut(&account_unique_id)
            .map(|mut field_unit| field_unit
                .get_total_energy_count_of_unit(field_unit_index as usize))
    }

    async fn get_field_unit_race_energy(&self,
                                        account_unique_id: i32,
                                        field_unit_index: i32,
                                        race_enum: RaceEnum) -> Option<i32> {
        let mut game_field_unit_repository_guard =
            self.game_field_unit_repository.lock().await;

        game_field_unit_repository_guard
            .get_game_field_unit_map()
            .get_mut(&account_unique_id)
            .map(|mut field_unit| field_unit
                .get_get_attached_energy_count_of_field_unit_with_race(field_unit_index as usize,
                                                                       race_enum))
    }

    async fn get_player_round(&self,
                              account_unique_id: i32) -> Option<i32> {
        let mut game_round_repository_guard =
            self.game_round_repository.lock().await;

        game_round_repository_guard
            .get_game_round_map()
            .get(&account_unique_id)
            .map(|user_round| user_round.get_round())
    }
}

#[async_trait]
impl GameFieldUnitActionValidatorService for GameFieldUnitActionValidatorServiceImpl {
    async fn is_unit_basic_attack_possible(
        &self, is_unit_basic_attack_possible_request: IsUnitBasicAttackPossibleRequest)
        -> IsUnitBasicAttackPossibleResponse {

        println!("GameFieldUnitActionValidatorServiceImpl: is_unit_basic_attack_possible()");

        // 1. check unit turn action
        let turn_action = self.get_field_unit_turn_action(
            is_unit_basic_attack_possible_request.get_account_unique_id(),
            is_unit_basic_attack_possible_request.get_field_unit_index()).await.unwrap_or(true);

        if turn_action == true {
            println!("이번 턴에 더 이상 액션이 불가능합니다.");
            return IsUnitBasicAttackPossibleResponse::new(false)
        }

        // 2. check round
        let field_unit_deployed_round = self.get_field_unit_deployed_round(
            is_unit_basic_attack_possible_request.get_account_unique_id(),
            is_unit_basic_attack_possible_request.get_field_unit_index()).await.unwrap_or(-1);

        let player_current_round = self.get_player_round(
            is_unit_basic_attack_possible_request.get_account_unique_id()).await.unwrap_or(-1);

        if player_current_round == field_unit_deployed_round {
            println!("소환된 턴에는 액션이 불가합니다.");
            return IsUnitBasicAttackPossibleResponse::new(false)
        }

        // 3. check energy enough
        let total_energy_count_of_field_unit = self.get_total_energy_count_of_field_unit(
            is_unit_basic_attack_possible_request.get_account_unique_id(),
            is_unit_basic_attack_possible_request.get_field_unit_index()).await.unwrap_or(-1);

        if is_unit_basic_attack_possible_request
            .get_basic_attack_required_energy_count() > total_energy_count_of_field_unit {
            println!("기본 공격에 필요한 에너지가 충분하지 않습니다.");
            return IsUnitBasicAttackPossibleResponse::new(false)
        }

        IsUnitBasicAttackPossibleResponse::new(true)
    }
}