use std::time::Instant;

use histo::Histogram;
// use rand::{SeedableRng, Rng};
// use rand::rngs::StdRng;
use table_games::blackjack::{Blackjack, BlackjackConfig, BlackjackState};
use table_games::blackjack::policies::{BasicPolicy, CountingPolicy};

// use ndarray::prelude::*;
use ndarray::Array;
use table_games::utils::MAIN_BROKER;

fn main() {
    println!("Hello, world!");

    let start = Instant::now();

    let hour_count = 0_250_000;
    let rounds_per_hour = 100;

    let mut results: Array<f32, _> = Array::zeros(hour_count);
    let mut histogram = Histogram::with_buckets(40);

    for hour_idx in 0..hour_count {

        // println!("Hour: {} ", hour_idx);

        MAIN_BROKER.lock().unwrap().clear();

        let config = BlackjackConfig::standard();
        let mut game = Blackjack::new(config);

        // let player = Box::new(BasicPolicy::new());
        // let player = Box::new(CountingPolicy::new());
        let player = CountingPolicy::new();

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
        histogram.add((game.get_player(0).get_bank() + 5000.0) as u64);
    }

    println!("Average: ${} +/- {}", results.mean().unwrap(), results.std(1.));
    println!("Elapsed: {} ms for {} hours", start.elapsed().as_millis(), hour_count);

    println!("{}", histogram);
}
