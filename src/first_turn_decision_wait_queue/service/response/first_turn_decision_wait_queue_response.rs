use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstTurnDecisionWaitQueueResponse {
    is_success: bool
}

impl FirstTurnDecisionWaitQueueResponse {
    pub fn new(is_success: bool) -> Self {
        FirstTurnDecisionWaitQueueResponse { is_success }
    }
    pub fn get_is_success(&self) -> bool { self.is_success }

}

