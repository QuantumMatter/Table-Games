from .blackjack import *

basic_strategy = {
    'hard': {
        #     2    3    4    5    6    7    8    9   10    A
        17: ['S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', 'S'],
        16: ['S', 'S', 'S', 'S', 'S', 'H', 'H', 'H', 'H', 'H'],
        15: ['S', 'S', 'S', 'S', 'S', 'H', 'H', 'H', 'H', 'H'],
        14: ['S', 'S', 'S', 'S', 'S', 'H', 'H', 'H', 'H', 'H'],
        13: ['S', 'S', 'S', 'S', 'S', 'H', 'H', 'H', 'H', 'H'],
        12: ['H', 'H', 'S', 'S', 'S', 'H', 'H', 'H', 'H', 'H'],
        11: ['DH', 'DH', 'DH', 'DH', 'DH', 'DH', 'DH', 'DH', 'DH', 'DH'],
        10: ['DH', 'DH', 'DH', 'DH', 'DH', 'DH', 'DH', 'DH', 'H', 'H'],
         9: ['H', 'DH', 'DH', 'DH', 'DH', 'H', 'H', 'H', 'H', 'H'],
         8: ['H', 'H', 'H', 'H', 'H', 'H', 'H', 'H', 'H', 'H'],
    },
    'soft': {
        #     2    3    4    5    6    7    8    9   10    A
        20: ['S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', 'S', 'S'], # A9
        19: ['S', 'S', 'S', 'S', 'DS', 'S', 'S', 'S', 'S', 'S'], # A8
        18: ['DS', 'DS', 'DS', 'DS', 'DS', 'S', 'S', 'H', 'H', 'H'], # A7
        17: ['H', 'DH', 'DH', 'DH', 'DH', 'H', 'H', 'H', 'H', 'H'], # A6
        16: ['H', 'H', 'DH', 'DH', 'DH', 'H', 'H', 'H', 'H', 'H'], # A5
        15: ['H', 'H', 'DH', 'DH', 'DH', 'H', 'H', 'H', 'H', 'H'], # A4
        14: ['H', 'H', 'H', 'DH', 'DH', 'H', 'H', 'H', 'H', 'H'], # A3
        13: ['H', 'H', 'H', 'DH', 'DH', 'H', 'H', 'H', 'H', 'H'], # A2
    },
    'pairs': {
        #     2    3    4    5    6    7    8    9   10    A
         1: ['Y', 'Y', 'Y', 'Y', 'Y', 'Y', 'Y', 'Y', 'Y', 'Y'],
        10: ['N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N'],
         9: ['Y', 'Y', 'Y', 'Y', 'Y', 'N', 'Y', 'Y', 'N', 'N'],
         8: ['Y', 'Y', 'Y', 'Y', 'Y', 'Y', 'Y', 'Y', 'Y', 'Y'],
         7: ['Y', 'Y', 'Y', 'Y', 'Y', 'Y', 'N', 'N', 'N', 'N'],
         6: ['Y', 'Y', 'Y', 'Y', 'Y', 'N', 'N', 'N', 'N', 'N'],
         5: ['N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N'],
         4: ['N', 'N', 'N', 'Y', 'Y', 'N', 'N', 'N', 'N', 'N'],
         3: ['Y', 'Y', 'Y', 'Y', 'Y', 'Y', 'N', 'N', 'N', 'N'],
         2: ['Y', 'Y', 'Y', 'Y', 'Y', 'Y', 'N', 'N', 'N', 'N'],
    }
}


def has_ace(cards: List[Card]):
    for card in cards:
        if card._value == 1:
            return True
        
    return False


class BasicPolicy(PlayerPolicy):

    @classmethod
    def PrebetAction(cls, player: PlayerState, submit):
        submit(PlayerSpreadAction(1))

    @classmethod
    def Bet(cls, player: PlayerState, submit):
        submit(10)
    
    @classmethod
    def InsuranceAction(cls, player: PlayerState) -> bool:
        return False
    
    @classmethod
    def Action(cls, player: PlayerState, spot: SpotState, up_card: Card, submit):

        action = ""
        
        is_pair = True
        is_pair &= len(spot._cards) == 2
        is_pair &= spot._cards[0]._value == spot._cards[1]._value

        soft = soft_total(spot._cards)
        hard = hard_total(spot._cards)

        column = min(10, up_card._value) - 2
        if column == -1: column = 9

        if is_pair:
            should_split = basic_strategy['pairs'][min(10, spot._cards[0]._value)][column]
            if should_split == 'Y':
                if not submit(SpotSplitAction()): raise Exception()
                return
        
        if (soft <= 21) and (has_ace(spot._cards)) and (soft != hard):
            soft = max(13, soft)
            soft = min(soft, 20)
            action = basic_strategy['soft'][soft][column]
        else:
            hard = max(8, hard)
            hard = min(hard, 17)
            action = basic_strategy['hard'][hard][column]
        
        if action == 'S': submit(SpotStandAction())
        elif action == 'H': submit(SpotHitAction())
        elif action[0] == 'D':
            if not submit(SpotDoubleAction()):
                if action[1] == 'S':
                    submit(SpotStandAction())
                elif action[1] == 'H':
                    submit(SpotHitAction())
                else:
                    raise Exception()
                

class TestOutPolicy(PlayerPolicy):

    @classmethod
    def PrebetAction(cls, player: PlayerState, submit):
        submit(PlayerSpreadAction(1))

    @classmethod
    def Bet(cls, player: PlayerState, submit):
        submit(10)
    
    @classmethod
    def InsuranceAction(cls, player: PlayerState) -> bool:
        return False

    @classmethod
    def Action(cls, player: PlayerState, spot: SpotState, up_card: Card, submit):
        def cli_submit(cli_action):

            def basic_submit(basic_action):
                if type(cli_action) != type(basic_action):
                    print('Wrong choice!')

                return submit(cli_action)

            BasicPolicy.Action(player, spot, up_card, basic_submit)
            return True

        CLIPlayer.Action(player, spot, up_card, cli_submit)


if __name__ == "__main__":

    from tqdm import tqdm

    game = Blackjack(6, True, 5, 5, 100)
    # game.add_player(TestOutPolicy())

    # while game.next():
    #     if game._state == BlackjackState.PREBETTING:
    #         print(f'You have ${ game._players[0][1]._bank }')
    #     elif game._state == BlackjackState.ACTION:
    #         print(f'Dealer is showing: { game._dealer[0] }')
        

    # exit()

    game.add_player(BasicPolicy())
    # game.add_player(BasicPolicy())
    # game.add_player(BasicPolicy())

    for _ in tqdm(range(6 * 100 * 500 * 20 * 10 * 5 * 3)):
        if not game.next():
            raise Exception()
        
    print('Done!')

    for idx, (playerPolicy, playerState) in enumerate(game._players):
        print(f'Player { idx+1 } won ${ playerState._bank }')
