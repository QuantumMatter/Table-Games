from enum import Enum
from typing import List
import random

class CSuit(Enum):
    CLUBS = 1
    DIAMONDS = 2
    HEARTS = 3
    SPADES = 4


class CValue(Enum):
    ACE = 1
    TWO = 2
    THREE = 3
    FOUR = 4
    FIVE = 5
    SIX = 6
    SEVEN = 7
    EIGHT = 8
    NINE = 9
    TEN = 10
    JACK = 11
    QUEEN = 12
    KING = 13


class Card:

    def __init__(self, suit: int, value: int) -> None:
        self._suit = suit
        self._value = value

    def __repr__(self) -> str:
        suit_map = {
            CSuit.CLUBS.value:    "♧",
            CSuit.DIAMONDS.value: "♢",
            CSuit.HEARTS.value:   "♡",
            CSuit.SPADES.value:   "♤"
        }

        value_map = {
            CValue.ACE.value:     "A",
            CValue.JACK.value:    "J",
            CValue.QUEEN.value:   "Q",
            CValue.KING.value:    "K"
        }

        suit = suit_map[self._suit]
        value = self._value if self._value not in value_map else value_map[self._value]

        return f"{suit}{value}"


class Deck:

    def __init__(self, cards: List[Card]) -> None:
        self._cards = cards

    def __add__(self, o):
        new_cards = [*self._cards, *o._cards]
        return Deck(new_cards)

    def shuffle(self):
        random.shuffle(self._cards)

    def draw(self):
        return self._cards.pop(0)
    
    def __len__(self):
        return len(self._cards)

    @classmethod
    def Standard(cls):
        cards = []
        
        for suit in CSuit:
            for value in CValue:
                cards.append(Card(suit.value, value.value))

        return Deck(cards)
    

suit_map = {
    'C': CSuit.CLUBS.value,
    'S': CSuit.SPADES.value,
    'H': CSuit.HEARTS.value,
    'D': CSuit.DIAMONDS.value
}
value_map = {
    'A': CValue.ACE.value,
    'K': CValue.KING.value,
    'Q': CValue.QUEEN.value,
    'J': CValue.JACK.value,
    'T': CValue.TEN.value,
    '9': CValue.NINE.value,
    '8': CValue.EIGHT.value,
    '7': CValue.SEVEN.value,
    '6': CValue.SIX.value,
    '5': CValue.FIVE.value,
    '4': CValue.FOUR.value,
    '3': CValue.THREE.value,
    '2': CValue.TWO.value
}