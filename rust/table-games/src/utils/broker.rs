use std::{rc::Rc, sync::{Arc, Mutex, MutexGuard}, ops::{DerefMut, Deref}, cell::RefCell};

use lazy_static::lazy_static;

use crate::common::Card;

#[derive(Clone)]
pub enum Message {
    NewShoe,
    CardDrawn(Card),
}

pub trait Observer: Sync+Send {
    fn OnMessage(&mut self, message: Message);
}

pub struct Broker {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
}
lazy_static! {
    pub static ref MAIN_BROKER: Arc<Mutex<Broker>> = Arc::new(Mutex::new(Broker { observers: vec![] }));
}


impl Broker {

    pub fn add_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    pub fn post(&self, message: Message) {
        for observer in self.observers.iter() {
            // observer.OnMessage(message.clone());
            // observer.borrow_mut().OnMessage(message.clone());
            unsafe { observer.as_ptr().as_mut().unwrap().OnMessage(message.clone()); }
        }
    }

    pub fn clear(&mut self) {
        self.observers.clear();
    }
}

unsafe impl Send for Broker {}
unsafe impl Sync for Broker {}