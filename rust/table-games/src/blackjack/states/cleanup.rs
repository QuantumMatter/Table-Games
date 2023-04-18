use crate::blackjack::{Blackjack, BlackjackState, BlackjackStateHandler, BlackjackError};

pub struct CleanupStateHandler {}
impl BlackjackStateHandler for CleanupStateHandler {
    fn execute(_game: &mut Blackjack) -> Result<BlackjackState, BlackjackError> {
        Ok(BlackjackState::Prebetting)
    }
}