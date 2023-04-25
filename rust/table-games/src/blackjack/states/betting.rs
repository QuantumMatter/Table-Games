use crate::blackjack::{Blackjack, BlackjackState, BlackjackStateHandler, BlackjackError};

pub struct BettingStateHandler {}
impl BlackjackStateHandler for BettingStateHandler {
    fn execute(game: &mut Blackjack) -> Result<BlackjackState, BlackjackError> {

        let tmin = game.config.tmin;
        let tmax: u32 = game.config.tmax;

        for (_p_idx, player) in game.players.iter_mut().enumerate() {

            if player.state.spots.len() == 0 {
                continue
            }

            let cloned = player.state.clone();
            let policy = &player.policy;

            let mut submit: Box<dyn FnMut(u32) -> bool> = Box::new(|bet: u32| {

                if bet > tmax {
                    return false
                } else if bet < tmin {
                    return false
                }

                player.state.bet(bet as f32);

                true
            });

            policy.bet(&cloned, &mut submit);
        }

        Ok(BlackjackState::Dealing)
    }
}


// I can fix the above lifetime issue by changing the following:
//
