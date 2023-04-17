from random import shuffle

# import sys
# import os
# sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'src'))

from table_games.common.cards import *
from table_games.blackjack.blackjack import Blackjack, BlackjackState
from table_games.blackjack.basic import *

class TestCase:
    
    def __init__(self, title: str, deck: List[Card], results: List[float]) -> None:
        self._title = title
        self._deck = deck
        self.results = results

cases = [
    TestCase("Hit, Stand, & Double against 6",
        deck=[
			'CA', 'D4', 'H7', 'H6', 'CJ', 'DT',     'S6',
			'D5', 'D2', 'CT', 'H5', 'CJ', 'D2',     'ST',

			'H4',   # Spot 1 Doubles, stands on soft 20
			'CT',   # Spot 2 Hits, stands on hard 16
					# Spot 3 Stands, on hard 17
			'C8',   # Spot 4 Doubles, stands on hard 19
					# Spot 5 Stands on hard 20
					# Spot 6 Stands, on hard 12

			'D8',   # Dealer draws to hard 24, busts
        ],
        results=[20, 10, 10, 20, 10, 10]
    ),
    TestCase("Hit, Stand, Double, Split against 8",
		deck=[
			'C4', 'DK', 'S5', 'H8', 'S8', 'H7',     'C8',
			'CJ', 'DK', 'C5', 'H7', 'DQ', 'D7',     'HQ',

			'H8',   # Spot 1 hits, busts with hard 22
					# Spot 2 stands, on hard 20
			'CQ',   # Spot 3 doubles, stands on hard 20
			'C7',   # Spot 4 hits, busts with hard 22
					# Spot 5 stands, on hard 18
			'D3',   # Spot 6 hits, stands on hard 17
		],
        results=[-10, 10, 20, -10, 0, -10]
	),
    TestCase("Blackjack, Hit, Double, Split, Stand, Push against 2 -> 21",
		deck=[
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
		],
        results=[15, -10, 0, -20 - 10, -10, 0]
	),
    TestCase("Splitting Aces against a Ten",
		deck=[
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
		],
        results=[-10 + 0, 0, (10 - 10) - 10, -10 + 10, -20, -10]
        # results = [-10, 0, -10, 0, -20, -10]
	),
    TestCase("Dealer draws soft 17, then soft 19",
		deck=[
			'H7', 'SK', 'C2', 'DA', 'ST', 'HJ',     'H6',
			'C8', 'DQ', 'S3', 'CJ', 'HA', 'D9',     'HA',

					# Spot 1 stands on hard 15
					# Spot 2 stands on hard 20
			'H9',   # Spot 3 hits hard 5, stands on hard 14
					# Spot 4 has blackjack
					# Spot 5 has blackjack
					# Spot 6 stands on hard 19

			'D2',   # Dealer hits soft 17, stands on soft 19
		],
        results=[-10, 10, -10, 15, 15, 0]
	),
    TestCase("Dealer busts after drawing to soft 17",
		deck=[
			'SJ', 'H3', 'DQ', 'CT', 'S5', 'HK',     'H3',
			'D4', 'CA', 'H7', 'S8', 'D2', 'C6',     'DA',

					# Spot 1 stands on 14
			'H2',   # Spot 2 hits soft 14
			'CK',   # Spot 2 hits soft 16, stands on hard 16
					# Spot 3 stands on 17
					# Spot 4 stands on 18
			'C7',   # Spot 5 hits 7, stands on 14
					# Spot 6 stands on 16

			'S3',   # Dealer hits soft 14, has soft 17
            'S6',   # Dealer hits soft 17, has hard 13 
			'CT',   # Dealer hits hard 13, busts
		],
        results=[10, 10, 10, 10, 10, 10]
	),
    TestCase("Pushes against soft total",
		deck=[
			'C9', 'DK', 'C3', 'HJ', 'S4', 'HA',     'C7',
			'DT', 'H6', 'SQ', 'D7', 'H5', 'C2',     'CA',

					# Spot 1 stands on 19
			'SA',   # Spot 2 hits 16, stands on 17
			'D6',   # Spot 3 hits 13, stands on 19,
					# Spot 4 stands on 17
			'H8',   # Spot 5 hits 9, stands on 17
			'D5',   # Spot 6 hits soft 13, stands on soft 18
		],
        results=[10, -10, 10, -10, -10, 0]
	),
    TestCase("Pairs against low up card",
		deck=[
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
		],
        results=[0 + -10, -10 - 10, -10, 20, -10 - 10, -20 - 10]
        # results=[-10, -20, -10, 20, -20, -20]
	),
    TestCase("Pairs against high up card",
		deck=[
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
		],
        results=[10, -10 + 0, -10 + 20, 0, 10, -20]
	),
    TestCase("Low soft totals aginst low up card",
		deck=[
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
		],
        results=[0, 10, -20, 20, -20, -20]
	),
    TestCase("Low totals that cannot double",
		deck=[
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
		],
        results=[10, -10, 10, 10, 10, -10]
	),
	TestCase("Dealer shows a 9",
		deck=[
			'H5', 'C9', 'DA', 'SA', 'CA', 'H3',		'D9',
			'D9', 'C2', 'D8', 'C4',	'CA', 'H3',		'DT',

			'H3',	# Spot 1 hits 14, stands on 17
			'S2',	# Spot 2 doubles on 11, stands on 13
					# Spot 3 stands on soft 19
			'SA',	# Spot 4 hits soft 15, has soft 16
			'S6',	# Spot 4 hits soft 16, has hard 12
			'S3',	# Spot 4 hits hard 12, has hard 15
			'H6',	# Spot 4 hits hard 15, stands on 21
					# Spot 5 splits A's
			'D2',	# Spot 5.1 draws 2, has 13
			'D9',	# Spot 5.2 draws 9, has 20
			'S8',	# Spot 6 hits 6, has 14
			'D4',	# Spot 6 hits 14, stands on 18
		],
		results=[-10, -20, 0, 10, -10 + 10, -10]
		# results=[-10, -20, 0, 10, 0, -10]
	),
	TestCase("Dealer shows an A",
	  	deck=[
			'H5', 'HA', 'D6', 'D4', 'D9', 'HA',		'DA',
			'C8', 'C3', 'DA', 'D4', 'C9', 'H2',		'D7',

			'S9', # Spot 1 hits 13, has 22
			'DJ', # Spot 2 hits soft 14, has hard 14
			'S5', # Spot 2 hits hard 14, has hard 19
			'HT', # Spot 3 hits soft 17, has hard 17
			'D3', # Spot 4 hits 8, has 11
			'S7', # Spot 4 hits 11, has 18
					# Spot 5 stands on 18
			'D5', # Spot 6 hits soft 13, has soft 18
			'DK', # Spot 6 hits soft 18, has hard 18
		],
		results=[-10, 10, -10, 0, 0, 0]
	),
	TestCase("Dealer shows a 5",
		deck=[
			'H5', 'C9', 'DA', 'SA', 'C9', 'H3',		'D5',
			'D5', 'C2', 'D8', 'C4',	'C9', 'H3',		'DT',

			'H3',	# Spot 1 doubles on 10, stands on 13
			'S8',	# Spot 2 doubles on 11, stands on 19
					# Spot 3 stands on soft 19
			'SA',	# Spot 4 hits doubles 15, has soft 16
					# Spot 5 splits 9's
			'D4',	# Spot 5.1 draws 4, has 13
			'D9',	# Spot 5.2 draws 9, has 18
					# Spot 5.2 splits 9's
			'D6',	# Spot 5.2.1 draws 6, has 15
			'DT',	# Spot 5.2.2 draws 10, has 19
					# Spot 6 splits 3's
			'D2',	# Spot 6.1 draws 2, has 5
			'D3',	# Spot 6.1 draws 3, has 8
			'D4',	# Spot 6.1 draws 4, stands on 12
			'S7',	# Spot 6.2 draws 7, has 10
			'D4',	# Spot 6.2 doubles on 10, stands on 14

			'H7',	# Dealer hits 15, busts with 22
		],
		results=[20, 20, 10, 20, 30, 30]
	),
	TestCase("Pairs against a 3",
	  	deck=[
			'H5', 'HA', 'D6', 'D4', 'D9', 'H8',		'D3',
			'H5', 'HA', 'D6', 'D4', 'D9', 'H8',		'DT',

			'S9', # Spot 1 doubles on 10, has 19
					# Spot 2 splits A's
			'D2', # Spot 2.1 draws 2, has 11
			'D9', # Spot 2.2 draws 9, has soft 20
					# Spot 3 splits 6's
			'S8', # Spot 3.1 draws 8, stands on 14
			'SK', # Spot 3.2 draws 10, has 16
			'D5', # Spot 4 hits on 8, stands on 13
					# Spot 5 splits 9's
			'D5', # Spot 5.1 draws 5, has 14
			'D6', # Spot 5.2 draws 6, has 15
					# Spot 6 splits 8's
			'H4', # Spot 6.1 draws 4, has 12
			'H4', # Spot 6.1 draws 4, has 16
			'CA', # Spot 6.2 draws A, has soft 19

			'CT', # Dealer hits 13, busts with 23
		],
		results=[20, 20, 20, 10, 20, 20]
	)
]

expectations = [0] * 6

def test_basic_strategy():
	PlayerPolicyMonitor.observing = BasicPolicy

	game = Blackjack(1, True, 1, 1, 100)
	for _ in range(6):
		# game.add_player(BasicPolicy())
		game.add_player(PlayerPolicyMonitor())
    
	for loop_idx in range(5):
		shuffle(cases)
                
		for case_idx, case in enumerate(cases):
			deck = Deck(list(map(lambda short: Card(suit_map[short[0]], value_map[short[1]]), case._deck)))
                                
			game._deck = deck
			for player_idx, player_bank_delta in enumerate(case.results):
				expectations[player_idx] += player_bank_delta

			game_finished = False
			while not game_finished:
				assert game.next(), f'Round { case_idx + 1 } failed!'
                                
				if game._state == BlackjackState.CLEANUP:
					print(f'Dealer has { game._dealer }')
					for p_idx, (p_policy, p_state) in enumerate(game._players):
						for s_idx, spot in enumerate(p_state._spots):
							print(f'Player { p_idx+1 }.{ s_idx+1 } has { spot._cards }')
					# pass
                                                        
				elif game._state == BlackjackState.PREBETTING:
					game_finished = True
                                        
					for player_idx, (player_expected, (playerPolicy, playerState)) in enumerate(zip(expectations, game._players)):
						assert player_expected == playerState._bank, f'Round { case_idx + 1 } failed! Player { player_idx + 1 } has ${ playerState._bank }, but should have ${ player_expected }'
                                                
					print(f'[OK] {loop_idx+1}.{case_idx+1}: "{case._title}" Passed!')

	PlayerPolicyMonitor.ToCsv('policy.csv')
			

if __name__ == "__main__":

	def make_submit_assertion(expected):
		
		def submit(action):
			if type(action) is SpotDoubleAction:
				return False
			assert type(action) is type(expected), f'Expected { type(expected) }, but got { type(action) }'
			return True
	
		return submit

	# BasicPolicy.Action(PlayerState(), SpotState.FromCards(['S6', 'D5']), Card('H', 2), make_submit_assertion(SpotDoubleAction()))
	BasicPolicy.Action(PlayerState(), SpotState.FromCards(['SA', 'S2', 'SA']), Card('H', 6), make_submit_assertion(SpotHitAction()))
	BasicPolicy.Action(PlayerState(), SpotState.FromCards(['SA', 'S2', 'SA']), Card('H', 5), make_submit_assertion(SpotHitAction()))
	BasicPolicy.Action(PlayerState(), SpotState.FromCards(['SA', 'S3', 'SA']), Card('H', 6), make_submit_assertion(SpotHitAction()))
	BasicPolicy.Action(PlayerState(), SpotState.FromCards(['SA', 'S3', 'SA']), Card('H', 5), make_submit_assertion(SpotHitAction()))
	# test_basic_strategy()