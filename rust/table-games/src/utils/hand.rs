use std::cmp::min;

use crate::common::{Card, CardValue};

pub trait Hand {

    fn soft_total(&self) -> u8;
    fn hard_total(&self) -> u8;
    fn is_blackjack(&self) -> bool;

}

impl Hand for Vec<Card> {

    fn soft_total(&self) -> u8 {
        let mut total = 0;
        let mut has_ace = false;

        for card in self.iter() {
            total += min(card.value() as u8, 10);
            if card.value() == CardValue::Ace {
                has_ace = true;
            }
        }

        if has_ace && total + 10 <= 21 {
            total += 10;
        }

        total
    }

    fn hard_total(&self) -> u8 {
        let mut total = 0;

        for card in self.iter() {
            total += min(card.value() as u8, 10);
        }

        total
    }

    fn is_blackjack(&self) -> bool {
        self.len() == 2 && self.soft_total() == 21
    }

}