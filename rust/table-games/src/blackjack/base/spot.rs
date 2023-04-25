use std::fmt::Display;

use crate::common::{Card};

#[derive(Clone)]
pub struct SpotState {
    pub bet: u32,
    pub cards: Vec<Card>,
    pub split: bool,
    pub insured: bool,
}

impl SpotState {
    pub fn new() -> SpotState {
        SpotState {
            bet: 0,
            cards: Vec::new(),
            split: false,
            insured: false,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum SpotAction {
    Hit,
    Stand,
    Double,
    Split,
}

pub trait SpotPolicy {
    fn action(&self, state: &SpotState) -> SpotAction;
}

impl Display for SpotState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let comma: String = ", ".to_string();
        

        write!(f, "Bet: {}, Cards:[{}]", self.bet, self.cards.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(&comma))
    }
}