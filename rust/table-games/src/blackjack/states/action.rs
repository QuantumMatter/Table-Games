use crate::blackjack::{Blackjack, BlackjackState, BlackjackStateHandler, BlackjackError};

pub struct ActionStateHandler {}
impl BlackjackStateHandler for ActionStateHandler {
    fn execute(_game: &mut Blackjack) -> Result<BlackjackState, BlackjackError> {
        Ok(BlackjackState::Results)
    }
}