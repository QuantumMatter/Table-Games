use std::fmt::{Display, Formatter, Error};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter};
use rand::seq::SliceRandom;
use std::ops::AddAssign;
use std::vec::Vec;

use rand::SeedableRng;
use rand::rngs::StdRng;


#[derive(Debug, EnumIter, Clone, Copy)]
pub enum CardSuit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl CardSuit {
    pub fn from_short(short: &str) -> Option<CardSuit> {
        match short {
            "C" => Some(CardSuit::Clubs),
            "D" => Some(CardSuit::Diamonds),
            "H" => Some(CardSuit::Hearts),
            "S" => Some(CardSuit::Spades),
            _ => panic!("Invalid suit: {}", short),
        }
    }
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum CardValue {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

impl CardValue {
    pub fn from_short(short: &str) -> Option<CardValue> {
        match short {
            "A" => Some(CardValue::Ace),
            "2" => Some(CardValue::Two),
            "3" => Some(CardValue::Three),
            "4" => Some(CardValue::Four),
            "5" => Some(CardValue::Five),
            "6" => Some(CardValue::Six),
            "7" => Some(CardValue::Seven),
            "8" => Some(CardValue::Eight),
            "9" => Some(CardValue::Nine),
            "T" => Some(CardValue::Ten),
            "J" => Some(CardValue::Jack),
            "Q" => Some(CardValue::Queen),
            "K" => Some(CardValue::King),
            _ => panic!("Invalid value: {}", short),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    suit: CardSuit,
    value: CardValue,
}

impl Card {
    pub fn from_short(short: &str) -> Option<Card> {
        let suit = CardSuit::from_short(&short[0..1])?;
        let value = CardValue::from_short(&short[1..2])?;

        Some(Card { suit, value })
    }

    pub fn value(&self) -> CardValue {
        self.value
    }

    pub fn suit(&self) -> CardSuit {
        self.suit
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let suit = match self.suit {
            CardSuit::Clubs => "♣",
            CardSuit::Diamonds => "♦",
            CardSuit::Hearts => "♥",
            CardSuit::Spades => "♠",
        };

        let value = match self.value {
            CardValue::Ace => "A",
            CardValue::Two => "2",
            CardValue::Three => "3",
            CardValue::Four => "4",
            CardValue::Five => "5",
            CardValue::Six => "6",
            CardValue::Seven => "7",
            CardValue::Eight => "8",
            CardValue::Nine => "9",
            CardValue::Ten => "10",
            CardValue::Jack => "J",
            CardValue::Queen => "Q",
            CardValue::King => "K",
        };

        write!(f, "{}{}", value, suit)
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();

        for suit in CardSuit::iter() {
            for value in CardValue::iter() {
                cards.push(Card { suit, value });
            }
        }

        Deck { cards }
    }

    pub fn from_cards(cards: Vec<Card>) -> Deck {
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        // let mut rng = rand::thread_rng();
        let mut rng = StdRng::from_entropy();
        self.cards.shuffle(&mut rng);
        self.cards.reverse();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn add(self, rhs: Self) -> Self {
        Deck { cards: [&self.cards[..], &rhs.cards[..]].concat() }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

}

impl AddAssign for Deck {
    fn add_assign(&mut self, rhs: Self) {
        self.cards.extend(rhs.cards);
    }
}

#[test]
fn test_deck() {
    // println!("{}", Deck::new().cards.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "));
    assert_eq!(52, Deck::new().len());

    let mut two_decks = Deck::new();
    two_decks += Deck::new();
    assert_eq!(104, two_decks.len());

    two_decks.shuffle();
    assert_eq!(104, two_decks.len());
    
    println!("{}", two_decks.cards.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "));
}