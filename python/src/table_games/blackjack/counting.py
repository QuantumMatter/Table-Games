from tqdm import tqdm

from .basic import BasicPolicy, PlayerState, SpotState, Card
from ..utils.broker import Observer, Broker, CardDrawMessage, NewShoeMessage

class AdvantagedPlayer(BasicPolicy, Observer):

    COUNT = 0
    
    def __new__(cls):
        instance = super().__new__(cls)
        Broker.MAIN().add_observer(instance)
        return instance
    
    def OnMessage(self, message):
        if isinstance(message, CardDrawMessage):
            card: Card = message.card
            if card._value in [2, 3, 4, 5, 6]:
                AdvantagedPlayer.COUNT += 1
            elif card._value in [1, 10, 11, 12, 13]:
                AdvantagedPlayer.COUNT -= 1
            else:
                pass
        elif isinstance(message, NewShoeMessage):
            AdvantagedPlayer.COUNT = 0
            # print('New Shoe')
            
            # print(f'Card Drawn: {card} ({AdvantagedPlayer.COUNT})')
    

    @classmethod
    def Bet(cls, game: 'Blackjack', player: PlayerState, submit):
        running_count = cls.COUNT
        true_count = running_count / (len(game._deck) / 52)
        multiplier = min(8, max(1, int(true_count)))
        # print(f'Bet ${ 10 * multiplier } @ {running_count}/{(len(game._deck) / 52):.2f}={true_count:.2f}')
        submit(10 * multiplier)


if __name__ == '__main__':
    from ..blackjack import *
    import numpy as np

    hour_count = 1000
    rounds_per_hour = 100
    hour_results = np.zeros(hour_count)

    for i in tqdm(range(hour_count)):
        
        game = Blackjack(6, True, 5/6, 10, 100)
        game.add_player(AdvantagedPlayer())

        round_count = 0
        while round_count < rounds_per_hour:
            assert game.next()
            if game._state == BlackjackState.CLEANUP:
                round_count += 1

        playerPolicy, playerState = game._players[0]
        hour_results[i] = playerState._bank

    
    print(f'Average bank: ${np.mean(hour_results)} +/- ${np.std(hour_results)}')

    # for player_idx, (playerPolicy, playerState) in enumerate(game._players):
    #     print(f'Player {player_idx} has ${playerState._bank}')
