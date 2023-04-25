use std::{cmp::min};

use crate::common::{Card, CardValue};

pub trait Hand {

    fn soft_total(&self) -> u8;
    fn hard_total(&self) -> u8;
    fn is_blackjack(&self) -> bool;

    fn pretty(&self) -> String;
    fn from_short(short_cards: Vec<&str>) -> Self;

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

    fn pretty(&self) -> String {
        let comma: String = ", ".to_string();
        self.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(&comma)
    }

    fn from_short(short_cards: Vec<&str>) -> Self {
        let mut deck = Vec::new();

        for short_card in short_cards.iter() {
            deck.push(Card::from_short(short_card).unwrap());
        }

        deck
    }

}
