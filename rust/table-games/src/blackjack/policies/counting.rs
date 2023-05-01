use std::{rc::Rc, cmp::{min, max}, sync::Mutex, cell::RefCell};

use crate::{blackjack::{PlayerPolicy, Blackjack}, utils::{Broker, Observer, MAIN_BROKER, Message}, common::{CardValue, Deck}};

use super::BasicPolicy;

pub struct CountingPolicy
{
    running_count: i32,
    root_policy: BasicPolicy
}

impl Observer for CountingPolicy {
    fn OnMessage(&mut self, message: crate::utils::Message) {
        match message {
            Message::CardDrawn(c) => {
                if [CardValue::Ten, CardValue::Jack, CardValue::Queen, CardValue::King, CardValue::Ace].contains(&c.value()) {
                    self.running_count -= 1;
                } else if [CardValue::Two, CardValue::Three, CardValue::Four, CardValue::Five, CardValue::Six].contains(&c.value()) {
                    self.running_count += 1;
                }
            },
            _ => ()
        }
    }
}

impl CountingPolicy {
    pub fn new() -> Rc<RefCell<CountingPolicy>> {
        // let s = CountingPolicy {
        //     root_policy: BasicPolicy::new(),
        //     running_count: 0
        // };

        // let test = Box::new(CountingPolicy {
        //     root_policy: BasicPolicy::new(),
        //     running_count: 0
        // });

        // let s_rc = Rc::new(Mutex::new(s));

        // MAIN_BROKER.lock().unwrap().add_observer(s_rc.clone());

        // s_rc

        let test = Rc::new(RefCell::new(CountingPolicy {
            root_policy: BasicPolicy::new(),
            running_count: 0
        }));

        MAIN_BROKER.lock().unwrap().add_observer(test.clone());

        test
    }
}

impl PlayerPolicy for CountingPolicy
{
    fn prebet_action<'a>(&self, state: &crate::blackjack::base::PlayerState, submit: &mut Box<dyn FnMut(crate::blackjack::base::PlayerAction) -> bool + 'a>) {
        self.root_policy.prebet_action(state, submit);
    }

    fn bet<'a>(&self, deck: &Deck, _state: &crate::blackjack::base::PlayerState, submit: &mut Box<dyn FnMut(u32) -> bool + 'a>) {
        let running_count = self.running_count as f32;
        let cards_remaining = deck.len() as f32;
        let decks_remaining = cards_remaining / 52.0;
        let true_count = running_count / decks_remaining;
        let true_count = f32::min(8.0, f32::max(1.0, true_count));
        let bet = 10 * (true_count as u32);
        submit(bet);
        // println!("${} @ {}\t({} / {} = {})", bet, true_count as u32, running_count, decks_remaining, true_count)
    }

    fn insurance_action(&self, state: &crate::blackjack::base::PlayerState) -> bool {
        self.root_policy.insurance_action(state)
    }

    fn action<'a>(&self, state: &crate::blackjack::base::PlayerState, spot: &crate::blackjack::base::SpotState, up_card: crate::common::Card, submit: &mut Box<dyn FnMut(crate::blackjack::base::SpotAction) -> bool + 'a>) {
        self.root_policy.action(state, spot, up_card, submit);
    }
}