use crate::common::{Card};

#[derive(Clone)]
pub struct SpotState {
    bet: u32,
    cards: Vec<Card>,
    split: bool,
    insured: bool,
}

pub enum SpotAction {
    Hit,
    Stand,
    Double,
    Split,
}

pub trait SpotPolicy {
    fn Action(&self, state: &SpotState) -> SpotAction;
}