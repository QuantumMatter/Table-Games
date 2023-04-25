use std::time::Instant;

// use rand::{SeedableRng, Rng};
// use rand::rngs::StdRng;
use table_games::blackjack::{Blackjack, BlackjackConfig, BlackjackState};
use table_games::blackjack::policies::BasicPolicy;

// use ndarray::prelude::*;
use ndarray::Array;

fn main() {
    println!("Hello, world!");

    let start = Instant::now();

    let hour_count = 1000000;
    let rounds_per_hour = 100;

    let mut results: Array<f32, _> = Array::zeros(hour_count);

    for hour_idx in 0..hour_count {

        // println!("Hour: {} ", hour_idx);

        let config = BlackjackConfig::standard();
        let mut game = Blackjack::new(config);

        let player = Box::new(BasicPolicy::new());

        game.add_player(player);

        let mut round_idx = 0;
        while round_idx < rounds_per_hour {
            match game.next() {
                Ok(_state) => {
                    if game.get_state() == BlackjackState::Prebetting {
                        // println!("Round: {}", round_idx);
                        round_idx += 1;
                    }
                },
                Err(_err) => {
                    break;
                }
            }
        }

        results[hour_idx] = game.get_player(0).get_bank();
    }

    println!("Average: ${} +/- {}", results.mean().unwrap(), results.std(1.));
    println!("Elapsed: {} ms for {} hours", start.elapsed().as_millis(), hour_count);
}
