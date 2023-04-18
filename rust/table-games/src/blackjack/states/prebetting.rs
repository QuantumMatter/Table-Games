use crate::blackjack::policies::{SpotState, PlayerAction};
use crate::blackjack::{Blackjack, BlackjackState, BlackjackStateHandler, BlackjackError};

pub struct PrebettingStateHandler {}
impl BlackjackStateHandler for PrebettingStateHandler {
    fn execute(game: &mut Blackjack) -> Result<BlackjackState, BlackjackError> {
        
        for (player_idx, player) in game.players.iter_mut().enumerate() {
            
            let cloned = player.state.clone();
            let policy = &player.policy;

            let mut submit: Box<dyn FnMut(PlayerAction) -> bool> = Box::new(|action| {

                let spots = match action {
                    PlayerAction::Spread(spots) => spots,
                };

                println!("Player {} spread to {} spots", player_idx, spots);

                for _ in 0..spots {
                    player.state.spots.push(SpotState::new());
                }

                true
            });

            policy.prebet_action(&cloned, &mut submit);
        }

        Ok(BlackjackState::Betting)
    }
}