use std::cmp::min;

use super::super::SpotAction;
use crate::utils::Hand;
use crate::{
    blackjack::{
        policies::SpotState, Blackjack, BlackjackError, BlackjackState, BlackjackStateHandler,
    },
    common::CardValue,
};

pub struct ActionStateHandler {}
impl BlackjackStateHandler for ActionStateHandler {
    fn execute(game: &mut Blackjack) -> Result<BlackjackState, BlackjackError> {
        for (_player_idx, player) in game.players.iter_mut().enumerate() {
            let cloned = player.state.clone();
            let policy = &player.policy;

            let mut spot_idx = 0;
            while spot_idx < player.state.get_spots().len() {
                // let spot = &mut player.state.get_spots()[spot_idx];
                let cloned_spot = player.state.get_spots()[spot_idx].clone();

                if player.state.get_spots()[spot_idx].cards.len() == 1 {
                    player.state.get_spots()[spot_idx]
                        .cards
                        .push(game.deck.draw().expect("No cards left in deck"));
                }

                let split_aces = player.state.get_spots()[spot_idx].split
                    && player.state.get_spots()[spot_idx].cards[0].value() == CardValue::Ace;
                let dealt_second_ace =
                    player.state.get_spots()[spot_idx].cards[1].value() == CardValue::Ace;

                let mut has_action = !split_aces;
                has_action |= dealt_second_ace && game.config.rsa;
                has_action &= !player.state.get_spots()[spot_idx].cards.is_blackjack();

                if !has_action {
                    spot_idx += 1;
                    continue;
                }

                let mut done = false;
                let mut new_spot: Option<SpotState> = None;

                while !done {
                    let mut submit: Box<dyn FnMut(SpotAction) -> bool> =
                        Box::new(|action: SpotAction| {
                            match action {
                                SpotAction::Stand => {
                                    done = true;
                                    true
                                }
                                SpotAction::Hit => {
                                    player.state.get_spots()[spot_idx]
                                        .cards
                                        .push(game.deck.draw().expect("No cards left in deck"));
                                    if player.state.get_spots()[spot_idx].cards.hard_total() > 21 {
                                        done = true;
                                    }
                                    true
                                }
                                SpotAction::Double => {
                                    if player.state.get_spots()[spot_idx].cards.len() != 2 {
                                        return false;
                                    }
                                    if player.state.get_spots()[spot_idx].split && !game.config.das
                                    {
                                        return false;
                                    }

                                    player.state.get_spots()[spot_idx]
                                        .cards
                                        .push(game.deck.draw().expect("No cards left in deck"));
                                    player.state.bank -=
                                        player.state.get_spots()[spot_idx].bet as f32;
                                    player.state.get_spots()[spot_idx].bet *= 2;

                                    done = true;
                                    true
                                }
                                SpotAction::Split => {
                                    if player.state.get_spots()[spot_idx].cards.len() != 2 {
                                        return false;
                                    }

                                    let a_value = min(
                                        10,
                                        player.state.get_spots()[spot_idx].cards[0].value() as u8,
                                    );
                                    let b_value = min(
                                        10,
                                        player.state.get_spots()[spot_idx].cards[1].value() as u8,
                                    );
                                    if a_value != b_value {
                                        return false;
                                    }

                                    let mut local_spot = SpotState::new();
                                    local_spot.split = true;
                                    local_spot.split = true;

                                    local_spot.cards.push(
                                        player.state.get_spots()[spot_idx].cards.pop().unwrap(),
                                    );

                                    local_spot.bet = player.state.get_spots()[spot_idx].bet;
                                    player.state.bank -= local_spot.bet as f32;

                                    // player.state.spots.insert(spot_idx, new_spot);

                                    let _ = new_spot.insert(local_spot);
                                    spot_idx -= 1;
                                    done = true;

                                    true
                                }
                            }
                        });

                    policy.action(&cloned, &cloned_spot, &mut submit);
                }

                spot_idx += 1;
            }
        }

        Ok(BlackjackState::Results)
    }
}
