use crate::{blackjack::{Blackjack, BlackjackState, BlackjackStateHandler, BlackjackError}, utils::Hand};

pub struct ResultsStateHandler {}
impl BlackjackStateHandler for ResultsStateHandler {
    fn execute(game: &mut Blackjack) -> Result<BlackjackState, BlackjackError> {

        let mut must_draw = false;

        for player in game.players.iter() {
            for spot in player.state.spots.iter() {
                if spot.cards.is_blackjack() {
                    continue;
                }
                if spot.cards.hard_total() > 21 {
                    continue;
                }
                must_draw = true;
            }
        }

        if must_draw {
            loop {
                let hard = game.dealer.hard_total();
                let soft = game.dealer.soft_total();

                if hard >= 17 {
                    break;
                }
                if soft >= 18 {
                    break;
                }

                game.dealer.push(game.deck.draw().expect("No cards left in deck"));
            }
        }

        let dealer_total = game.dealer.soft_total();

        for player in game.players.iter_mut() {
            for spot in player.state.spots.iter_mut() {
                let spot_total = spot.cards.soft_total();

                if !spot.split && spot.cards.is_blackjack() {
                    player.state.bank += (1.0 + game.config.bj) * spot.bet as f32;
                } else if spot_total > 21 {
                    // pass
                } else if dealer_total > 21 {
                    player.state.bank += (2 * spot.bet) as f32;
                } else if spot_total > dealer_total {
                    player.state.bank += (2 * spot.bet) as f32;
                } else if spot_total == dealer_total {
                    player.state.bank += spot.bet as f32;
                } else if spot_total < dealer_total {
                    // pass
                } else {
                    panic!("Unexpected result");
                }
            }
        }

        Ok(BlackjackState::Cleanup)
    }
}