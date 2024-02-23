use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_card_support::service::request::summarize_support_card_effect_request::{SummarizeSupportCardEffectRequest};
use crate::game_card_support_usage_counter::service::request::check_support_card_usage_count_request::CheckSupportCardUsageCountRequest;
use crate::game_card_support_usage_counter::service::request::update_support_card_usage_count_request::UpdateSupportCardUsageCountRequest;
use crate::game_deck::service::request::draw_cards_from_deck_request::DrawCardsFromDeckRequest;
use crate::game_deck::service::request::game_deck_card_draw_request::GameDeckCardDrawRequest;
use crate::game_deck::service::request::game_deck_card_shuffle_request::GameDeckCardShuffleRequest;
use crate::game_hand::service::request::add_card_list_to_hand_request::AddCardListToHandRequest;
use crate::game_hand::service::request::use_game_hand_support_card_request::UseGameHandSupportCardRequest;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_support_card_request::IsItSupportCardRequest;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_draw_support_card_request::NotifyToOpponentYouUseDrawSupportCardRequest;
use crate::notify_player_action_info::service::request::notice_draw_card_by_using_hand_card_request::NoticeDrawCardByUsingHandCardRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

#[derive(Debug)]
pub struct DrawSupportRequestForm {
    session_id: String,
    support_card_id: String,
}

impl DrawSupportRequestForm {
    pub fn new(session_id: &str, support_card_id: &str) -> Self {
        DrawSupportRequestForm {
            session_id: session_id.to_string(),
            support_card_id: support_card_id.to_string()
        }
    }
    pub fn get_support_card_id(&self) -> &str { &self.support_card_id }
    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }
    pub fn to_is_this_your_turn_request(&self,
                                        account_unique_id: i32) -> IsThisYourTurnRequest {
        IsThisYourTurnRequest::new(account_unique_id)
    }
    pub fn to_check_protocol_hacking_request(&self, account_unique_id: i32, support_card_number: i32) -> CheckProtocolHackingRequest {
        CheckProtocolHackingRequest::new(account_unique_id, support_card_number)
    }
    pub fn to_can_use_card_request(&self, account_unique_id: i32, support_card_number: i32) -> CanUseCardRequest {
        CanUseCardRequest::new(account_unique_id, support_card_number)
    }
    pub fn to_is_it_support_card_request(&self, support_card_number: i32) -> IsItSupportCardRequest {
        IsItSupportCardRequest::new(support_card_number)
    }
    pub fn to_check_support_card_usage_count_request(&self, account_unique_id: i32) -> CheckSupportCardUsageCountRequest {
        CheckSupportCardUsageCountRequest::new(account_unique_id)
    }
    pub fn to_update_support_card_usage_count_request(&self, account_unique_id: i32) -> UpdateSupportCardUsageCountRequest {
        UpdateSupportCardUsageCountRequest::new(account_unique_id)
    }
    pub fn to_use_game_hand_support_card_request(&self, account_unique_id: i32, support_card_number: i32) -> UseGameHandSupportCardRequest {
        UseGameHandSupportCardRequest::new(account_unique_id, support_card_number)
    }
    pub fn to_place_to_tomb_request(&self, account_unique_id: i32, used_card_id: i32) -> PlaceToTombRequest {
        PlaceToTombRequest::new(account_unique_id, used_card_id)
    }
    pub fn to_summarize_support_card_effect_request(&self, support_card_number: i32) -> SummarizeSupportCardEffectRequest {
        SummarizeSupportCardEffectRequest::new(support_card_number)
    }
    pub fn to_find_opponent_by_account_id_request(&self, account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(account_unique_id)
    }
    pub fn to_notice_draw_card_by_using_hand_card_request(&self,
                                                          account_unique_id: i32,
                                                          opponent_unique_id: i32,
                                                          used_hand_card_id: i32,
                                                          drawn_card_list: Vec<i32>) -> NoticeDrawCardByUsingHandCardRequest {
        NoticeDrawCardByUsingHandCardRequest::new(
            account_unique_id,
            opponent_unique_id,
            used_hand_card_id,
            drawn_card_list)
    }
    pub fn to_draw_cards_from_deck_request(&self, account_unique_id: i32, draw_count: i32) -> DrawCardsFromDeckRequest {
        DrawCardsFromDeckRequest::new(account_unique_id, draw_count)
    }
    pub fn to_add_card_list_to_hand_request(&self, account_unique_id: i32, card_list: Vec<i32>) -> AddCardListToHandRequest {
        AddCardListToHandRequest::new(account_unique_id, card_list)
    }
    pub fn to_shuffle_deck_request(&self) -> GameDeckCardShuffleRequest {
        GameDeckCardShuffleRequest::new( self.session_id.clone() )
    }
}