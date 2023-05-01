use crate::{blackjack::{Blackjack, BlackjackState, BlackjackStateHandler, BlackjackError}, utils::Hand, common::CardValue};

pub struct DealingStateHandler {}
impl BlackjackStateHandler for DealingStateHandler {
    fn execute(game: &mut Blackjack) -> Result<BlackjackState, BlackjackError> {

        for player in game.players.iter_mut() {
            for spot in player.state.spots.iter_mut() {
                spot.cards.push(game.deck.draw(false).expect("No cards left in deck"));
            }
        }

        game.dealer.push(game.deck.draw(false).expect("No cards left in deck"));

        for player in game.players.iter_mut() {
            for spot in player.state.spots.iter_mut() {
                spot.cards.push(game.deck.draw(false).expect("No cards left in deck"));
            }
        }

        game.dealer.push(game.deck.draw(false).expect("No cards left in deck"));

        if game.dealer[0].value() == CardValue::Ace {
            for player in game.players.iter_mut() {

                let takes_insurance = player.policy.borrow().insurance_action(&player.state);
                if takes_insurance {
                    for spot in player.state.spots.iter_mut() {
                        player.state.bank -= (spot.bet as f32) / 2.0;
                        spot.insured = true;
                    }
                }

            }
        }

        if game.dealer.is_blackjack() {
            for player in game.players.iter_mut() {
                for spot in player.state.spots.iter_mut() {
                    if spot.insured {
                        player.state.bank += (3.0 / 2.0) * (spot.bet as f32);
                    }
                }
            }
            return Ok(BlackjackState::Cleanup);
        }

        Ok(BlackjackState::Action)
    }
}