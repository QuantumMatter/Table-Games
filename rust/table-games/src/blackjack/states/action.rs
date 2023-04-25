use std::cmp::min;

use super::super::SpotAction;
use crate::utils::Hand;
use crate::{
    blackjack::{
        base::SpotState, Blackjack, BlackjackError, BlackjackState, BlackjackStateHandler,
    },
    common::CardValue,
};

pub struct ActionStateHandler {}
impl BlackjackStateHandler for ActionStateHandler {
    fn execute(game: &mut Blackjack) -> Result<BlackjackState, BlackjackError> {
        for (player_idx, player) in game.players.iter_mut().enumerate() {

            // println!("-----------------");

            let mut spot_idx: i8 = 0;
            while (spot_idx as usize) < player.state.get_spots().len() {

                if player.state.get_spots()[spot_idx as usize].cards.len() == 1 {
                    player.state.get_spots()[spot_idx as usize]
                        .cards
                        .push(game.deck.draw().expect("No cards left in deck"));
                }

                let split_aces = player.state.get_spots()[spot_idx as usize].split
                    && player.state.get_spots()[spot_idx as usize].cards[0].value() == CardValue::Ace;
                let dealt_second_ace =
                    player.state.get_spots()[spot_idx as usize].cards[1].value() == CardValue::Ace;

                let mut has_action = !split_aces;
                has_action |= dealt_second_ace && game.config.rsa;
                has_action &= !player.state.get_spots()[spot_idx as usize].cards.is_blackjack();

                if !has_action {
                    // println!("Spot {}.{} SKIPPED with [{}] vs {}", player_idx+1, spot_idx+1, player.state.get_spots()[spot_idx as usize].cards.pretty(), game.dealer.pretty());
                    spot_idx += 1;
                    continue;
                }

                let mut done = false;
                let mut new_spot: Option<SpotState> = None;

                while !done {

                    let cloned = player.state.clone();
                    let policy = &player.policy;
                    
                    // let spot = &mut player.state.get_spots()[spot_idx];
                    let cloned_spot = player.state.get_spots()[spot_idx as usize].clone();

                    let mut submit: Box<dyn FnMut(SpotAction) -> bool> =
                        Box::new(|action: SpotAction| {

                            // let action_str = match action {
                            //     SpotAction::Stand => "STANDS",
                            //     SpotAction::Hit => "HITS",
                            //     SpotAction::Double => "DOUBLES",
                            //     SpotAction::Split => "SPLITS",
                            // };
    
                            // println!("Spot {}.{} {} on [{}] vs {}", player_idx+1, spot_idx+1, action_str, player.state.spots[spot_idx as usize].cards.pretty(), game.dealer[0]);

                            match action {
                                SpotAction::Stand => {
                                    done = true;
                                    true
                                }
                                SpotAction::Hit => {
                                    player.state.get_spots()[spot_idx as usize]
                                        .cards
                                        .push(game.deck.draw().expect("No cards left in deck"));
                                    if player.state.get_spots()[spot_idx as usize].cards.hard_total() > 21 {
                                        done = true;
                                    }
                                    true
                                }
                                SpotAction::Double => {
                                    if player.state.get_spots()[spot_idx as usize].cards.len() != 2 {
                                        return false;
                                    }
                                    if player.state.get_spots()[spot_idx as usize].split && !game.config.das
                                    {
                                        return false;
                                    }

                                    player.state.get_spots()[spot_idx as usize]
                                        .cards
                                        .push(game.deck.draw().expect("No cards left in deck"));
                                    player.state.bank -=
                                        player.state.get_spots()[spot_idx as usize].bet as f32;
                                    player.state.get_spots()[spot_idx as usize].bet *= 2;

                                    done = true;
                                    true
                                }
                                SpotAction::Split => {
                                    if player.state.get_spots()[spot_idx as usize].cards.len() != 2 {
                                        return false;
                                    }

                                    let a_value = min(10, player.state.get_spots()[spot_idx as usize].cards[0].value() as u8);
                                    let b_value = min(10, player.state.get_spots()[spot_idx as usize].cards[1].value() as u8);
                                    if a_value != b_value {
                                        return false;
                                    }

                                    let mut local_spot = SpotState::new();
                                    player.state.get_spots()[spot_idx as usize].split = true;
                                    local_spot.split = true;

                                    local_spot.cards.push(
                                        player.state.get_spots()[spot_idx as usize].cards.pop().unwrap(),
                                    );
                                    // player.state.get_spots()[spot_idx as usize].cards.push(
                                    //     game.deck.draw().expect("No cards left in deck"),
                                    // );
                                    // local_spot.cards.push(
                                    //     game.deck.draw().expect("No cards left in deck"),
                                    // );

                                    local_spot.bet = player.state.get_spots()[spot_idx as usize].bet;
                                    player.state.bank -= local_spot.bet as f32;

                                    player.state.spots.insert((spot_idx+1) as usize, local_spot);

                                    // let _ = new_spot.insert(local_spot);
                                    spot_idx -= 1;
                                    done = true;

                                    true
                                }
                            }
                        });

                    policy.action(&cloned, &cloned_spot, game.dealer[0], &mut submit);
                }

                spot_idx += 1;
            }
        }

        Ok(BlackjackState::Results)
    }
}
