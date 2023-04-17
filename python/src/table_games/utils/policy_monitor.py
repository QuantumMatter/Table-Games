from table_games.common.cards import *
from table_games.blackjack.blackjack import PlayerPolicy, PlayerState, SpotState, SpotSplitAction, SpotDoubleAction, SpotHitAction, SpotStandAction, soft_total, hard_total


class Wrapper:
    def __init__(self, value) -> None:
        self.value = value


class PlayerPolicyMonitor(PlayerPolicy):

    policy = {
        'hard':  { idx: ['']*10 for idx in range(8, 18)  },
        'soft':  { idx: ['']*10 for idx in range(13, 21) },
        'pairs': { idx: ['']*10 for idx in range(1, 11)  },
    }
    
    observing: PlayerPolicy = None

    @classmethod
    def Action(cls, player: PlayerState, spot: SpotState, up_card: Card, submit):

        attempted_double = Wrapper(False)

        def m_submit(action):
            
            is_pair = True
            is_pair &= len(spot._cards) == 2
            is_pair &= spot._cards[0]._value == spot._cards[1]._value

            soft = soft_total(spot._cards)
            hard = hard_total(spot._cards)

            column = min(10, up_card._value) - 2
            if column == -1: column = 9

            m_action = ''
            if   type(action) is SpotSplitAction:  m_action = 'V'
            elif type(action) is SpotDoubleAction: m_action = 'D'
            elif type(action) is SpotHitAction:    m_action = 'H'
            elif type(action) is SpotStandAction:  m_action = 'S'
            else: raise Exception('Unknown action type')

            if attempted_double.value:
                m_action = 'D' + m_action

            if submit(action):
                if is_pair:
                    row = min(10, spot._cards[0]._value)
                    if type(action) is SpotSplitAction:
                        cls.policy['pairs'][row][column] = 'Y'
                        return True
                    else:
                        cls.policy['pairs'][row][column] = 'N'
                    
                if soft != hard:
                    row = min(20, soft)
                    cls.policy['soft'][row][column] = m_action
                else:
                    row = max(8, min(17, hard))
                    cls.policy['hard'][row][column] = m_action

                return True

            elif type(action) is SpotDoubleAction:
                attempted_double.value = True
                return False
            
            else:
                raise Exception('Unknown action type')
            
        
        cls.observing.Action(player, spot, up_card, m_submit)

    @classmethod
    def ToCsv(cls, filename):
        header = 'total,2,3,4,5,6,7,8,9,10,A'
        with open(filename, 'w') as f:
            f.write(header + '\n')
            f.write('\n')

            # Write the hard policy to the csv
            for row in reversed(range(8, 18)):
                f.write(f'{ row },')
                f.write(','.join(cls.policy['hard'][row]))
                f.write('\n')
                
            f.write('\n')
            
            # Write the soft policy to the csv
            for row in reversed(range(13, 21)):
                f.write(f'{ row },')
                f.write(','.join(cls.policy['soft'][row]))
                f.write('\n')

            f.write('\n')

            # Write the pair policy to the csv
            f.write('A,')
            f.write(','.join(cls.policy['pairs'][1]))
            f.write('\n')
            for row in reversed(range(2, 11)):
                f.write(f'{ row },')
                f.write(','.join(cls.policy['pairs'][row]))
                f.write('\n')

    @classmethod
    def PrebetAction(cls, player: PlayerState, submit):
        return cls.observing.PrebetAction(player, submit)
    
    @classmethod
    def Bet(cls, game: 'Blackjack', player: PlayerState, submit):
        return cls.observing.Bet(game, player, submit)
    
    @classmethod
    def InsuranceAction(cls, player: PlayerState) -> bool:
        return cls.observing.InsuranceAction(player)
        