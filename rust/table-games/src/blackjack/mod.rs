use std::{rc::Rc, sync::Mutex, cell::RefCell};

use crate::common::{Card, Deck};

mod states;
mod base;
pub mod policies;

use states::*;
use base::*;


pub struct Ploppy {}
impl PlayerPolicy for Ploppy {
    fn prebet_action<'a>(&self, _state: &PlayerState, submit: &mut Box<dyn FnMut(PlayerAction) -> bool + 'a>) {
        submit(PlayerAction::Spread(1));
    }

    fn bet<'a>(&self, _deck: &Deck, _state: &PlayerState, submit: &mut Box<dyn FnMut(u32) -> bool + 'a>) {

        println!("Ploppy is betting...");

        let mut value = 0;
        while ! submit(value) {
            value += 1;
        }
    }

    fn insurance_action(&self, _state: &PlayerState) -> bool {
        false
    }

    fn action<'a>(&self, _state: &PlayerState, _spot: &SpotState, _up_card: Card, submit: &mut Box<dyn FnMut(SpotAction) -> bool + 'a>) {
        submit(SpotAction::Stand);
    }
}

pub enum BlackjackError {
    UnknownState,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BlackjackState {
    Prebetting = 1,
    Betting = 2,
    Dealing = 3,
    Action = 4,
    Results = 5,
    Cleanup = 6,
}

trait BlackjackStateHandler {
    fn execute(game: &mut Blackjack) -> Result<BlackjackState, BlackjackError>;
}

// #[derive(Clone, Copy)]
pub struct BlackjackConfig {
    deck_count: usize,
    // h17: bool,
    pen: f32,
    tmin: u32,
    tmax: u32,

    das: bool,
    rsa: bool,
    // spc: u8,

    bj: f32
}

impl BlackjackConfig {
    pub fn standard() -> BlackjackConfig {
        BlackjackConfig {
            deck_count: 6,
            // h17: true,
            pen: 5.0/6.0,
            tmin: 10,
            tmax: 200,

            das: true,
            rsa: true,
            // spc: 4,

            bj: 1.5
        }
    }
}

pub struct Player {

    // policy: Rc<Mutex<dyn PlayerPolicy>>,
    // policy: Box<dyn PlayerPolicy>,
    policy: Rc<RefCell<dyn PlayerPolicy>>,
    state: PlayerState,

}

pub struct Blackjack {

    config: BlackjackConfig,
    state: BlackjackState,

    players: Vec<Player>,
    deck: Deck,
    dealer: Vec<Card>,
}

impl Blackjack {
    pub fn new(config: BlackjackConfig) -> Blackjack {
        let mut deck = Deck::new();
        for _ in 1..config.deck_count {
            deck += Deck::new();
        }
        deck.shuffle();

        let _ = deck.draw(true);

        Blackjack {
            config,
            state: BlackjackState::Prebetting,
            players : vec![],
            // players: vec![Player { policy: Box::new(Ploppy {}), state: PlayerState { spots: vec![], bank: 1000 } }]
            deck,
            dealer: vec![]
        }
    }

    pub fn next(&mut self) -> Result<(), BlackjackError> {
        let next_state = match self.state {
            BlackjackState::Prebetting => PrebettingStateHandler::execute(self),
            BlackjackState::Betting => BettingStateHandler::execute(self),
            BlackjackState::Dealing => DealingStateHandler::execute(self),
            BlackjackState::Action => ActionStateHandler::execute(self),
            BlackjackState::Results => ResultsStateHandler::execute(self),
            BlackjackState::Cleanup => CleanupStateHandler::execute(self),
        };

        match next_state {
            Ok(state) => self.state = state,
            Err(err) => return Err(err)
        }

        Ok(())
    }

    pub fn get_state(&self) -> BlackjackState {
        self.state
    }

    pub fn add_player(&mut self, policy: Rc<RefCell<dyn PlayerPolicy>>) {
        let state = PlayerState { spots: vec![], bank: 0.0 };
        self.players.push(Player { policy, state });
    }

    pub fn get_player(&self, index: usize) -> &PlayerState {
        &self.players[index].state
    }
}