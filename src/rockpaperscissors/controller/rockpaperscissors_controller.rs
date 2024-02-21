use async_trait::async_trait;

use crate::rockpaperscissors::controller::request_form::rockpaperscissors_request_form::RockpaperscissorsRequestForm;
use crate::rockpaperscissors::controller::response_form::rockpaperscissors_response_form::RockpaperscissorsResponseForm;
use crate::rockpaperscissors::controller::request_form::check_rockpaperscissors_winner_request_form::CheckRockpaperscissorsWinnerRequestForm;
use crate::rockpaperscissors::controller::response_form::check_rockpaperscissors_winner_response_form::CheckRockpaperscissorsWinnerResponseForm;

#[async_trait]
pub trait  RockpaperscissorsController {
    async fn execute_rockpaperscissors_procedure(
        &self, rockpaperscissors_request_form: RockpaperscissorsRequestForm) -> RockpaperscissorsResponseForm;
    async fn execute_check_winner_procedure(
        &self, check_rockpaperscissors_winner_request_form: CheckRockpaperscissorsWinnerRequestForm) -> CheckRockpaperscissorsWinnerResponseForm;
}