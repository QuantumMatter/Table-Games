from table_games.blackjack.blackjack import soft_total
from table_games.common.cards import *

def ez_best_total(cards):
    cards = list(map(lambda c: Card(CSuit.DIAMONDS, value_map[c]), cards))
    return soft_total(cards)

def test_best_total():
    assert ez_best_total(['A', '6', '3']) == 20
    assert ez_best_total(['6', '3', '4', '2', '5']) == 20
    assert ez_best_total(['K', 'A']) == 21
    assert ez_best_total(['5', 'A', 'K', '4']) == 20
    assert ez_best_total(['A', '9']) == 20
    assert ez_best_total(['3', 'A', '2', 'K']) == 16

if __name__ == "__main__":
    test_best_total()