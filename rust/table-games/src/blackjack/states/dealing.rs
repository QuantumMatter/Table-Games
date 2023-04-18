use crate::blackjack::{Blackjack, BlackjackState, BlackjackStateHandler, BlackjackError};

pub struct DealingStateHandler {}
impl BlackjackStateHandler for DealingStateHandler {
    fn execute(_game: &mut Blackjack) -> Result<BlackjackState, BlackjackError> {
        Ok(BlackjackState::Action)
    }
}