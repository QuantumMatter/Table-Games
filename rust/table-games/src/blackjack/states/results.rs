use crate::blackjack::{Blackjack, BlackjackState, BlackjackStateHandler, BlackjackError};

pub struct ResultsStateHandler {}
impl BlackjackStateHandler for ResultsStateHandler {
    fn execute(_game: &mut Blackjack) -> Result<BlackjackState, BlackjackError> {
        Ok(BlackjackState::Cleanup)
    }
}