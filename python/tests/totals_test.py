from table_games.blackjack.blackjack import best_total
from table_games.common.cards import *

def ez_best_total(cards, against):
    cards = list(map(lambda c: Card(CSuit.DIAMONDS, value_map[c]), cards))
    return best_total(cards, against)

def test_best_total():
    assert ez_best_total(['A', '6', '3'], 17) == 20
    assert ez_best_total(['6', '3', '4', '2', '5'], 19) == 20
    assert ez_best_total(['K', 'A'], 15) == 21
    assert ez_best_total(['5', 'A', 'K', '4'], 18) == 20
    assert ez_best_total(['A', '9'], 20) == 20
    assert ez_best_total(['3', 'A', '2', 'K'], 15) == 16

if __name__ == "__main__":
    test_best_total()