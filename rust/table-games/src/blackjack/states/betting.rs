use crate::blackjack::{Blackjack, BlackjackState, BlackjackStateHandler, BlackjackError};

pub struct BettingStateHandler {}
impl BlackjackStateHandler for BettingStateHandler {
    fn execute(game: &mut Blackjack) -> Result<BlackjackState, BlackjackError> {

        let tmin = game.config.tmin;
        let tmax: u32 = game.config.tmax;

        for (p_idx, player) in game.players.iter_mut().enumerate() {

            let cloned = player.state.clone();
            let policy = &player.policy;

            let mut submit: Box<dyn FnMut(u32) -> bool> = Box::new(|bet: u32| {
                
                println!("Player {} bet {} ", p_idx, bet);

                if bet > tmax {
                    return false
                } else if bet < tmin {
                    return false
                }

                // inner_p.lock().unwrap().state.bet(u128::from(bet));
                player.state.bet(u128::from(bet));

                true
            });

            policy.Bet(&cloned, &mut submit);
        }

        Ok(BlackjackState::Dealing)
    }
}


// I can fix the above lifetime issue by changing the following:
//
