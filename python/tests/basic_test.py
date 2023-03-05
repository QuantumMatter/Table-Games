from math import ceil

# import table_games
from table_games.common.cards import *
from table_games.blackjack.blackjack import Blackjack, BlackjackState, best_total
from table_games.blackjack.basic import BasicPolicy

# from cards import CSuit, CValue, Card, Deck
# from blackjack import Blackjack, BlackjackState, best_total
# from basic import BasicPolicy

# 6 spots, perfect basic strategy, $10/spot
d = [
    # Burn Card
    # 'D5',

    # Round 1 - Hit, Stand, & Double against 6
    'CA', 'D4', 'H7', 'H6', 'CJ', 'DT',     'S6',
    'D5', 'D2', 'CT', 'H5', 'CJ', 'D2',     'ST',

    'H4',   # Spot 1 Doubles, stands on soft 20
    'CT',   # Spot 2 Hits, stands on hard 16
            # Spot 3 Stands, on hard 17
    'C8',   # Spot 4 Doubles, stands on hard 19
            # Spot 5 Stands on hard 20
            # Spot 6 Stands, on hard 12

    'D8',   # Dealer draws to hard 24, busts

    # P1: $20, P2: $10, P3: $10, P4: $20, P5: $10, P6: $10
    
    # Round 2 - Hit, Stand, Double, Split against 8
    'C4', 'DK', 'S5', 'H8', 'S8', 'H7',     'C8',
    'CJ', 'DK', 'C5', 'H7', 'DQ', 'D7',     'HQ',

    'H8',   # Spot 1 hits, busts with hard 22
            # Spot 2 stands, on hard 20
    'CQ',   # Spot 3 doubles, stands on hard 20
    'C7',   # Spot 4 hits, busts with hard 22
            # Spot 5 stands, on hard 18
    'D3',   # Spot 6 hits, stands on hard 17

    # P1: $10, P2: $20, P3: $30, P4: $10, P5: $10, P6: $0

    # Round 3 - Blackjack, Hit, Double, Split, Stand, Push against 2 -> 21
    'CA', 'H4', 'S6', 'C3', 'HQ', 'D7',     'C2',
    'CK', 'D8', 'D5', 'D3', 'S3', 'H2',     'DT',

            # Spot 1 has a blackjack
    'CJ',   # Spot 2 hits, busts with hard 22
    'HJ',   # Spot 3 doubles, stands on hard 21
            # Spot 4 splits
    'H7',   # Spot 4(a) is dealt 7, has hard 10
    'C8',   # Spot 4(a) doubles; stands on hard 18
    'H9',   # Spot 4(b) is dealt 9, has hard 12
    'H3',   # Spot 4(b) hits, stands on hard 15
            # Spot 5 stands on hard 13
    'C2',   # Spot 6 hits; has 11
    'CT',   # Spot 6 hits; has 21

    'D9',   # Dealer draws to 21

    # P1: $25, P2: $10, P3: $30, P4: $-20, P5: $0, P6: $0

    # Round 4 - Splitting Aces against a Ten
    'CA', 'CT', 'CA', 'SA', 'H8', 'D7',     'HT',
    'DA', 'HT', 'HA', 'CA', 'D3', 'H7',     'DT',

            # Spot 1 splits aces
    'D2',   # Spot 1(a) is dealt 2, must stand on soft 13
    'H9',   # Spot 1(b) is dealt 9, must stand on hard 20
            # Spot 2 stands on hard 20
            # Spot 3 splits aces
    'HA',   # Spot 3(a) is dealt an Ace, and SPLITS again
    'CT',   # Spot 3(a)(a) is dealt 10, stands on soft 21
    'C5',   # Spot 3(a)(b) is dealt  5, stands on soft 16
    'H8',   # Spot 3(b)    is dealt  8, stands on soft 19
            # Spot 4 splits aces
    'S7',   # Spot 4(a)    is dealt  7, stands on soft 18
    'DT',   # Spot 4(b)    is dealt 10, stands on soft 21
    'H6',   # Spot 5 DOUBLES, has soft 17
    'S3',   # Spot 6 HITS, stands on hard 17

    # P1: $15, P2: $10, P3: $20, P4: $-20, P5: $-20, P6: $-10

    # Round 5 - Dealer draws soft 17, then soft 19
    'H7', 'SK', 'C2', 'DA', 'ST', 'HJ',     'H6',
    'C8', 'DQ', 'S3', 'CJ', 'HA', 'D9',     'HA',

            # Spot 1 stands on hard 15
            # Spot 2 stands on hard 20
    'H9',   # Spot 3 hits hard 5, stands on hard 14
            # Spot 4 has blackjack
            # Spot 5 has blackjack
            # Spot 6 stands on hard 19

    'D2',   # Dealer hits soft 17, stands on soft 19

    # P1: $5, P2: $20, P3: $10, P4: $-5, P5: $-5, $-10

    # Round 6 - Dealer busts after drawing to soft 17
    'SJ', 'H3', 'DQ', 'CT', 'S5', 'HK',     'H3',
    'D4', 'CA', 'H7', 'S8', 'D2', 'C6',     'DA',

            # Spot 1 stands on 14
    'H2',   # Spot 2 hits soft 14
    'CK',   # Spot 2 hits soft 16, stands on hard 16
            # Spot 3 stands on 17
            # Spot 4 stands on 18
    'C7',   # Spot 5 hits 7, stands on 14
            # Spot 6 stands on 16

    'ST',   # Dealer hits soft 14, has hard 14
    'CT',   # Dealer hits hard 14, busts

    # P1: $15, P2: $30, P3: $20, P4: $5, P5: $5, P6: $0

    # Round 7 - Pushes against soft total
    'C9', 'DK', 'C3', 'HJ', 'S4', 'HA',     'C7',
    'DT', 'H6', 'SQ', 'D7', 'H5', 'C2',     'CA',

            # Spot 1 stands on 19
    'SA',   # Spot 2 hits 16, stands on 17
    'D6',   # Spot 3 hits 13, stands on 19,
            # Spot 4 stands on 17
    'H8',   # Spot 5 hits 9, stands on 17
    'D5',   # Spot 6 hits soft 13, stands on soft 18

    # P1: $25, P2: $20, P3: $30, P4: $-5, P5: $-5, P6: $0
    
    # Round 8 - Pairs against low up card
    'C2', 'D3', 'H4', 'S5', 'C6', 'H7',     'D4',
    'C2', 'D3', 'H4', 'S5', 'C6', 'H7',     'D4',

            # Spot 1 splits 2's
    'C8',   # Spot 1(a) is dealt 8, has 10
    'H9',   # Spot 1(a) doubles on 10, stands on 19
    'C6',   # Spot 1(b) is dealt 6, has 8
    'CT',   # Spot 1(b) hits 8, stands on 18
            # Spot 2 splits 3's
    'CJ',   # Spot 2(a) is dealt 10, stands on 13
    'H4',   # Spot 2(b) is dealt 4
    'SK',   # Spot 2(b) hits 7, stands on 17
    'S7',   # Spot 3 hits 8, stands on 15
    'HQ',   # Spot 4 doubles on 10, stands on 20
            # Spot 5 splits 6's
    'S2',   # Spot 5(a) is dealt 2, has 8
    'HT',   # Spot 5(a) hits 8, stands on 18
    'D9',   # Spot 5(b) is dealt 9, stands on 15
            # Spot 6 splits 7's
    'D3',   # Spot 6(a) is dealt 3, has 10
    'C4',   # Spot 6(a) doubles on 10, stands on 14
    'C8',   # Spot 6(b) is dealt 15, stands on 15

    'DA',   # Dealer hits 8, stands on soft 19

    # P1: $15, P2: $0, P3: $20, P4: $15, P5: $-25, P6: $-30

    # Round 9 - Pairs against high up card
    'CT', 'D9', 'H8', 'S7', 'C6', 'H5',     'D8',
    'CT', 'D9', 'H8', 'S7', 'C6', 'H5',     'D8',

            # Spot 1 stands on 20
            # Spot 2 splits 9's
    'S6',   # Spot 2(a) is dealt 6, has 15
    'H9',   # Spot 2(a) hits 15, busts with 24
    'HK',   # Spot 2(b) dealt 10, stands on 19
            # Spot 3 splits 8's
    'H7',   # Spot 3(a) is dealt 7, has 15
    'CQ',   # Spot 3(a) hits 15, busts with 25
    'H3',   # Spot 3(b) is dealt 3, has 11
    'ST',   # Spot 3(b) doubles on 11, stands on 21
    'S5',   # Spot 4 hits 14, stands on 19
    'D8',   # Spot 5 hits 12, stands on 20
    'H2',   # Spot 6 doubles on 10, stands on 12

    'H3',   # Dealer hits 16, stands on 19

    # P1: $25, P2: $-10, P3: $30, P4: $15, P5: $-15, P6: $-50

    # Round 10 - Low soft totals aginst low up card
    'CA', 'CA', 'CA', 'CA', 'CA', 'CA',     'D4',
    'C2', 'C3', 'C4', 'C5', 'C6', 'C7',     'DT',

    'H5',   # Spot 1 hits soft 13, stands on soft 18
    'HA',   # Spot 2 hits soft 14, has soft 15
    'C6',   # Spot 2 hits soft 15, stands on soft 21
    'C7',   # Spot 3 doubles soft 15, stands on hard 12
    'S3',   # Spot 4 doubles soft 16, stands on soft 19
    'SJ',   # Spot 5 doubles soft 17, stands on hard 17
    'S8',   # Spot 6 doubles on soft 18, stands on hard 16

    'S4',   # Dealer hits 14, stands on 18

    # P1: $25, P2: $0, P3: $10, P4: $35, P5: $-35, P6: $-70

    # Round 11 - Low totals that cannot double
    'C2', 'D3', 'D5', 'D4', 'D6', 'D5',     'S6',
    'C4', 'D4', 'D2', 'D3', 'D2', 'D3',     'H5',

    'H3',   # Spot 1 hits 6, has 9
    'HA',   # Spot 1 hits 9, stands on soft 20
    'S3',   # Spot 2 hits 7, has 10
    'H6',   # Spot 2 hits 10, stands on 16
    'S4',   # Spot 3 hits 7, has 11
    'DK',   # Spot 3 hits 11, stands on 21
    'S2',   # Spot 4 hits 7, has 9
    'S9',   # Spot 4 hits 9, stands on 18
    'S2',   # Spot 5 hits 8, has 10
    'HQ',   # Spot 5 hits 10, stands on 20
    'S3',   # Spot 6 hits 8, has 11
    'D2',   # Spot 6 hits 11, stands on 13
    

    'S4',   # Dealer hits 11, has 15
    'S2',   # Dealer hits 15, stands on 17

    # P1: $35, P2: $-10, P3: $20, P4: $45, P5: $-25, P6: $-80

    # 'DK'
]

pen = len(d) / 52
deck_count = ceil(pen)

for _ in range(len(d), deck_count*52):
    d.append('DK')

expectations = [
    [0,  0,  0,  0,  0,  0],
    [20, 10, 10, 20, 10, 10],
    [10, 20, 30, 10, 10, 0],
    [25, 10, 30, -20, 0, 0],
    [15, 10, 20, -20, -20, -10],
    [5, 20, 10, -5, -5, -10],
    [15, 30, 20, 5, 5, 0],
    [25, 20, 30, -5, -5, 0],
    [15, 0, 20, 15, -25, -30],
    [25, -10, 30, 15, -15, -50],
    [25, 0, 10, 35, -35, -70],
    [35, -10, 20, 45, -25, -80]
]

def test_basic_strategy():

    cards = list(map(lambda short: Card(suit_map[short[0]], value_map[short[1]]), d))
    deck = Deck(cards)

    game = Blackjack(deck_count, True, pen, 1, 100)
    for _ in range(6):
        game.add_player(BasicPolicy())
    game._deck = deck

    round_idx = 0
    while True:
        if round_idx >= len(expectations):
            break

        if game._state == BlackjackState.PREBETTING:
            expectation = expectations[round_idx]
            for player_idx, (player_expected, (playerPolicy, playerState)) in enumerate(zip(expectation, game._players)):
                # assert player_expected == playerState._bank
                if player_expected != playerState._bank:
                    print(f'Round { round_idx + 1 } failed! Player { player_idx + 1 } has ${ playerState._bank }, but should have ${ player_expected }')
                    exit()
            print(f'Before Round { round_idx+1 } is OK')

        elif game._state == BlackjackState.ACTION:
            print(f'Dealer has { game._dealer }')
            for p_idx, (p_policy, p_state) in enumerate(game._players):
                for s_idx, spot in enumerate(p_state._spots):
                    print(f'Player { p_idx+1 }.{ s_idx+1 } has { spot._cards }')

        elif game._state == BlackjackState.CLEANUP:
            round_idx += 1
            print(f'Dealer has { game._dealer }')
            for p_idx, (p_policy, p_state) in enumerate(game._players):
                for s_idx, spot in enumerate(p_state._spots):
                    print(f'Player { p_idx+1 }.{ s_idx+1 } has { spot._cards }')

        game.next()

    print('Pass!')

print("Name:", __name__)

if __name__ == "__main__":
    test_basic_strategy()