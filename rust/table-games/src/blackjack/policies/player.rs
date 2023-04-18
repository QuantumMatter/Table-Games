use crate::blackjack::policies::spot::{SpotState, SpotAction};

#[derive(Clone)]
pub struct PlayerState {
    pub(crate) spots: Vec<SpotState>,
    pub(crate) bank: u128,
}

impl PlayerState {
    pub fn get_spots(&self) -> &Vec<SpotState> {
        &self.spots
    }

    pub fn bet(&mut self, bet: u128) {
        self.bank -= self.spots.len() as u128 * bet;
    }
}

pub enum PlayerAction {
    Spread(u8)
}

pub trait PlayerPolicy: 'static {
    fn PrebetAction(&self, state: &PlayerState) -> PlayerAction;
    fn Bet<'a>(&self, state: &PlayerState, submit: &mut Box<dyn FnMut(u32) -> bool + 'a>);
    fn InsuranceAction(&self, state: &PlayerState) -> bool;
    fn Action(&self, state: &PlayerState, spot: &SpotState, submit: Box<dyn FnOnce(SpotAction) -> bool>);
}