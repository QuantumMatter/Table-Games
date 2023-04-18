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

pub enum SpotAction {
    Hit,
    Stand,
    Double,
    Split,
}

pub trait SpotPolicy {
    fn action(&self, state: &SpotState) -> SpotAction;
}