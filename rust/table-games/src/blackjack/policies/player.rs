use crate::blackjack::policies::spot::{SpotState, SpotAction};

#[derive(Clone)]
pub struct PlayerState {
    pub(crate) spots: Vec<SpotState>,
    pub(crate) bank: f32,
}

impl PlayerState {
    pub fn get_spots(&mut self) -> &mut Vec<SpotState> {
        &mut self.spots
    }

    pub fn bet(&mut self, bet: f32) {
        for spot in self.spots.iter_mut() {
            self.bank -= bet;
            spot.bet = bet as u32;
        }
    }
}

pub enum PlayerAction {
    Spread(u8)
}

pub trait PlayerPolicy: 'static {
    fn prebet_action<'a>(&self, state: &PlayerState, submit: &mut Box<dyn FnMut(PlayerAction) -> bool + 'a>);
    fn bet<'a>(&self, state: &PlayerState, submit: &mut Box<dyn FnMut(u32) -> bool + 'a>);
    fn insurance_action(&self, state: &PlayerState) -> bool;
    fn action<'a>(&self, state: &PlayerState, spot: &SpotState, submit: &mut Box<dyn FnMut(SpotAction) -> bool + 'a>);
}