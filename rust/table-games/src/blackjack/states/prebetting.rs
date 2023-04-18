use crate::blackjack::{Blackjack, BlackjackState, BlackjackStateHandler, BlackjackError};

pub struct PrebettingStateHandler {}
impl BlackjackStateHandler for PrebettingStateHandler {
    fn execute(_game: &mut Blackjack) -> Result<BlackjackState, BlackjackError> {
        Ok(BlackjackState::Betting)
    }
}