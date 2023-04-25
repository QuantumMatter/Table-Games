use std::{collections::HashMap, cmp::{min, max}};

use crate::{blackjack::{base::{PlayerPolicy, PlayerAction, PlayerState, SpotState, SpotAction}, Blackjack, BlackjackConfig, BlackjackState}, utils::Hand, common::{Card, Deck}};

#[derive(Clone, Copy)]
enum BasicAction {
    Stand,
    Hit,
    Double(bool),
}

use BasicAction::Stand as S;
use BasicAction::Hit as H;

pub struct BasicPolicy {

    hard: HashMap<u8, [BasicAction; 10]>,
    soft: HashMap<u8, [BasicAction; 10]>,
    pairs: HashMap<u8, [bool; 10]>,

}

impl BasicPolicy {

    pub fn new() -> Self {

        #[allow(non_snake_case)]
        let Y = true;
        
        #[allow(non_snake_case)]
        let N = false;

        let Dh = BasicAction::Double(true);
        let Ds = BasicAction::Double(false);

        let hard = HashMap::from([
            (17, [S, S, S, S, S, S, S, S, S, S]),
            (16, [S, S, S, S, S, H, H, H, H, H]),
            (15, [S, S, S, S, S, H, H, H, H, H]),
            (14, [S, S, S, S, S, H, H, H, H, H]),
            (13, [S, S, S, S, S, H, H, H, H, H]),
            (12, [H, H, S, S, S, H, H, H, H, H]),
            (11, [Dh, Dh, Dh, Dh, Dh, Dh, Dh, Dh, Dh, Dh]),
            (10, [Dh, Dh, Dh, Dh, Dh, Dh, Dh, Dh, H, H]),
            (9,  [H, Dh, Dh, Dh, Dh, H, H, H, H, H]),
            (8,  [H, H, H, H, H, H, H, H, H, H]),
        ]);
        let soft = HashMap::from([
            (20, [S, S, S, S, S, S, S, S, S, S]),
            (19, [S, S, S, S, Ds, S, S, S, S, S]),
            (18, [Ds, Ds, Ds, Ds, Ds, S, S, H, H, H]),
            (17, [H, Dh, Dh, Dh, Dh, H, H, H, H, H]),
            (16, [H, H, Dh, Dh, Dh, H, H, H, H, H]),
            (15, [H, H, Dh, Dh, Dh, H, H, H, H, H]),
            (14, [H, H, H, Dh, Dh, H, H, H, H, H]),
            (13, [H, H, H, Dh, Dh, H, H, H, H, H]),
        ]);
        let pairs = HashMap::from([
            (1,  [Y, Y, Y, Y, Y, Y, Y, Y, Y, Y]),
            (10, [N, N, N, N, N, N, N, N, N, N]),
            (9,  [Y, Y, Y, Y, Y, N, Y, Y, N, N]),
            (8,  [Y, Y, Y, Y, Y, Y, Y, Y, Y, Y]),
            (7,  [Y, Y, Y, Y, Y, Y, N, N, N, N]),
            (6,  [Y, Y, Y, Y, Y, N, N, N, N, N]),
            (5,  [N, N, N, N, N, N, N, N, N, N]),
            (4,  [N, N, N, N, Y, Y, N, N, N, N]),
            (3,  [Y, Y, Y, Y, Y, Y, N, N, N, N]),
            (2,  [Y, Y, Y, Y, Y, Y, N, N, N, N]),
        ]);

        Self { hard, soft, pairs }
    }

}

impl PlayerPolicy for BasicPolicy {
    fn prebet_action<'a>(&self, _state: &PlayerState, submit: &mut Box<dyn FnMut(PlayerAction) -> bool + 'a>) {
        submit(PlayerAction::Spread(1));
    }

    fn bet<'a>(&self, _state: &PlayerState, submit: &mut Box<dyn FnMut(u32) -> bool + 'a>) {
        submit(10);
    }

    fn insurance_action(&self, _state: &PlayerState) -> bool {
        false
    }

    fn action<'a>(&self, _state: &PlayerState, spot: &SpotState, up_card: Card, submit: &mut Box<dyn FnMut(SpotAction) -> bool + 'a>) {

        let is_soft = spot.cards.soft_total() != spot.cards.hard_total();
        let can_split = spot.cards.len() == 2 && spot.cards[0].value() == spot.cards[1].value();

        // python code
        // column = min(10, up_card._value) - 2
        // if column == -1: column = 9

        // rust
        let mut column = min(10, up_card.value() as i8) - 2;
        if column == -1 { column = 9; }

        if can_split {
            let row_idx = min(spot.cards[0].value() as u8, 10);
            let row = self.pairs.get(&row_idx).expect(format!("[PAIRS] Row {}, Column {}", row_idx, column).as_str());
            
            if row[column as usize] {
                if !submit(SpotAction::Split) {
                    panic!("Split not allowed");
                }
                return;
            }
        }

        let action;

        if is_soft {
            let row_idx = min(spot.cards.soft_total() as u8, 20);
            let row = self.soft.get(&row_idx).expect(format!("[SOFT] Row {}, Column {}\t{:#?}", row_idx, column, spot.cards).as_str());
            action = &row[column as usize];
        } else {
            let row_idx = max(min(spot.cards.hard_total() as u8, 17), 8);
            let row = self.hard.get(&row_idx).expect(format!("[HARD] Row {}, Column {}", row_idx, column).as_str());
            action = &row[column as usize];
        }

        // let action_str = match action {
        //     SpotAction::Hit => "HIT",
        //     SpotAction::Stand => "STAND",
        //     SpotAction::Double => "DOUBLE",
        //     SpotAction::Split => "SPLIT",
        // };

        // println!("BASIC will {} on [{}] vs {}", action_str, spot.cards.pretty(), up_card);

        let _ = match action {
            BasicAction::Double(otherwise_hit) => {
                if !submit(SpotAction::Double) {
                    if *otherwise_hit {
                        if !submit(SpotAction::Hit) {
                            panic!("Unknown!");
                        }
                    }
                    else {
                        if !submit(SpotAction::Stand) {
                            panic!("Unknown!");
                        }
                    }
                }
                true
            },
            BasicAction::Hit => submit(SpotAction::Hit),
            BasicAction::Stand => submit(SpotAction::Stand),
            // BasicAction::Split => panic!("Split not allowed"),
        };

    }
}

#[test]
fn simple_test_basic_strategy() {

    let policy = BasicPolicy::new();


    let make_test = Box::new(|player_hand: Vec<Card>, up_card: Card, expected: SpotAction| {
        
        let mut game = Blackjack::new(BlackjackConfig::standard());
        game.add_player(Box::new(BasicPolicy::new()));

        game.players[0].state.spots.push(SpotState::new());
        game.players[0].state.spots[0].cards = player_hand;
        game.dealer = vec![up_card];

        let mut submit: Box<dyn FnMut(SpotAction) -> bool> = Box::new(move |action: SpotAction| {

            let action_str = match action {
                SpotAction::Hit => "Hit",
                SpotAction::Stand => "Stand",
                SpotAction::Double => "Double",
                SpotAction::Split => "Split",
            };

            println!("Testing BS! {}", action_str);
            assert_eq!(action, expected);
            // called = true;
            true
        });
        
        policy.action(&game.players[0].state, &game.players[0].state.spots[0], up_card, &mut submit);

    });

    make_test(Hand::from_short(vec!["S8", "S6", "S4"]), Card::from_short("S7").unwrap(), SpotAction::Stand);

}


struct TestCase {
    title: String,
    deck: Vec<String>,
    results: Vec<i16>
}

impl TestCase {
    fn new(title: &str, deck: Vec<&str>, results: Vec<i16>) -> Self {

        Self {
            title: title.to_string(),
            deck: deck.iter().map(|s| s.to_string()).collect(),
            results
        }
    }
}


#[test]
fn test_basic() {

    let cases = vec![
        TestCase::new("Hit, Stand, & Double against 6",
            vec![
                "CA", "D4", "H7", "H6", "CJ", "DT",     "S6",
                "D5", "D2", "CT", "H5", "CJ", "D2",     "ST",

                "H4",   // Spot 1 Doubles, stands on soft 20
                "CT",   // Spot 2 Hits, stands on hard 16
                        // Spot 3 Stands, on hard 17
                "C8",   // Spot 4 Doubles, stands on hard 19
                        // Spot 5 Stands on hard 20
                        // Spot 6 Stands, on hard 12

                "D8",   // Dealer draws to hard 24, busts
            ],
            vec![20, 10, 10, 20, 10, 10]
        ),
        TestCase::new("Hit, Stand, Double, Split against 8",
            vec![
                "C4", "DK", "S5", "H8", "S8", "H7",     "C8",
                "CJ", "DK", "C5", "H7", "DQ", "D7",     "HQ",

                "H8",   // Spot 1 hits, busts with hard 22
                        // Spot 2 stands, on hard 20
                "CQ",   // Spot 3 doubles, stands on hard 20
                "C7",   // Spot 4 hits, busts with hard 22
                        // Spot 5 stands, on hard 18
                "D3",   // Spot 6 hits, stands on hard 17
            ],
            vec![-10, 10, 20, -10, 0, -10]
        ),
        TestCase::new("Blackjack, Hit, Double, Split, Stand, Push against 2 -> 21",
            vec![
                "CA", "H4", "S6", "C3", "HQ", "D7",     "C2",
                "CK", "D8", "D5", "D3", "S3", "H2",     "DT",

                        // Spot 1 has a blackjack
                "CJ",   // Spot 2 hits, busts with hard 22
                "HJ",   // Spot 3 doubles, stands on hard 21
                        // Spot 4 splits
                "H7",   // Spot 4(a) is dealt 7, has hard 10
                "C8",   // Spot 4(a) doubles; stands on hard 18
                "H9",   // Spot 4(b) is dealt 9, has hard 12
                "H3",   // Spot 4(b) hits, stands on hard 15
                        // Spot 5 stands on hard 13
                "C2",   // Spot 6 hits; has 11
                "CT",   // Spot 6 hits; has 21

                "D9",   // Dealer draws to 21
            ],
            vec![15, -10, 0, -20 - 10, -10, 0]
        ),
        TestCase::new("Splitting Aces against a Ten",
            vec![
                "CA", "CT", "CA", "SA", "H8", "D7",     "HT",
                "DA", "HT", "HA", "CA", "D3", "H7",     "DT",

                        // Spot 1 splits aces
                "D2",   // Spot 1(a) is dealt 2, must stand on soft 13
                "H9",   // Spot 1(b) is dealt 9, must stand on hard 20
                        // Spot 2 stands on hard 20
                        // Spot 3 splits aces
                "HA",   // Spot 3(a) is dealt an Ace, and SPLITS again
                "CT",   // Spot 3(a)(a) is dealt 10, stands on soft 21
                "C5",   // Spot 3(a)(b) is dealt  5, stands on soft 16
                "H8",   // Spot 3(b)    is dealt  8, stands on soft 19
                        // Spot 4 splits aces
                "S7",   // Spot 4(a)    is dealt  7, stands on soft 18
                "DT",   // Spot 4(b)    is dealt 10, stands on soft 21
                "H6",   // Spot 5 DOUBLES, has soft 17
                "S3",   // Spot 6 HITS, stands on hard 17
            ],
            vec![-10 + 0, 0, (10 - 10) - 10, -10 + 10, -20, -10]
            // results = [-10, 0, -10, 0, -20, -10]
        ),
        TestCase::new("Dealer draws soft 17, then soft 19",
            vec![
                "H7", "SK", "C2", "DA", "ST", "HJ",     "H6",
                "C8", "DQ", "S3", "CJ", "HA", "D9",     "HA",

                        // Spot 1 stands on hard 15
                        // Spot 2 stands on hard 20
                "H9",   // Spot 3 hits hard 5, stands on hard 14
                        // Spot 4 has blackjack
                        // Spot 5 has blackjack
                        // Spot 6 stands on hard 19

                "D2",   // Dealer hits soft 17, stands on soft 19
            ],
            vec![-10, 10, -10, 15, 15, 0]
        ),
        TestCase::new("Dealer busts after drawing to soft 17",
            vec![
                "SJ", "H3", "DQ", "CT", "S5", "HK",     "H3",
                "D4", "CA", "H7", "S8", "D2", "C6",     "DA",

                        // Spot 1 stands on 14
                "H2",   // Spot 2 hits soft 14
                "CK",   // Spot 2 hits soft 16, stands on hard 16
                        // Spot 3 stands on 17
                        // Spot 4 stands on 18
                "C7",   // Spot 5 hits 7, stands on 14
                        // Spot 6 stands on 16

                "S3",   // Dealer hits soft 14, has soft 17
                "S6",   // Dealer hits soft 17, has hard 13 
                "CT",   // Dealer hits hard 13, busts
            ],
            vec![10, 10, 10, 10, 10, 10]
        ),
        TestCase::new("Pushes against soft total",
            vec![
                "C9", "DK", "C3", "HJ", "S4", "HA",     "C7",
                "DT", "H6", "SQ", "D7", "H5", "C2",     "CA",

                        // Spot 1 stands on 19
                "SA",   // Spot 2 hits 16, stands on 17
                "D6",   // Spot 3 hits 13, stands on 19,
                        // Spot 4 stands on 17
                "H8",   // Spot 5 hits 9, stands on 17
                "D5",   // Spot 6 hits soft 13, stands on soft 18
            ],
            vec![10, -10, 10, -10, -10, 0]
        ),
        TestCase::new("Pairs against low up card",
            vec![
                "C2", "D3", "H4", "S5", "C6", "H7",     "D4",
                "C2", "D3", "H4", "S5", "C6", "H7",     "D4",

                        // Spot 1 splits 2's
                "C8",   // Spot 1(a) is dealt 8, has 10
                "H9",   // Spot 1(a) doubles on 10, stands on 19
                "C6",   // Spot 1(b) is dealt 6, has 8
                "CT",   // Spot 1(b) hits 8, stands on 18
                        // Spot 2 splits 3's
                "CJ",   // Spot 2(a) is dealt 10, stands on 13
                "H4",   // Spot 2(b) is dealt 4
                "SK",   // Spot 2(b) hits 7, stands on 17
                "S7",   // Spot 3 hits 8, stands on 15
                "HQ",   // Spot 4 doubles on 10, stands on 20
                        // Spot 5 splits 6's
                "S2",   // Spot 5(a) is dealt 2, has 8
                "HT",   // Spot 5(a) hits 8, stands on 18
                "D9",   // Spot 5(b) is dealt 9, stands on 15
                        // Spot 6 splits 7's
                "D3",   // Spot 6(a) is dealt 3, has 10
                "C4",   // Spot 6(a) doubles on 10, stands on 14
                "C8",   // Spot 6(b) is dealt 15, stands on 15

                "DA",   // Dealer hits 8, stands on soft 19
            ],
            vec![0 + -10, -10 - 10, -10, 20, -10 - 10, -20 - 10]
            // vec![-10, -20, -10, 20, -20, -20]
        ),
        TestCase::new("Pairs against high up card",
            vec![
                "CT", "D9", "H8", "S7", "C6", "H5",     "D8",
                "CT", "D9", "H8", "S7", "C6", "H5",     "D8",

                        // Spot 1 stands on 20
                        // Spot 2 splits 9's
                "S6",   // Spot 2(a) is dealt 6, has 15
                "H9",   // Spot 2(a) hits 15, busts with 24
                "HK",   // Spot 2(b) dealt 10, stands on 19
                        // Spot 3 splits 8's
                "H7",   // Spot 3(a) is dealt 7, has 15
                "CQ",   // Spot 3(a) hits 15, busts with 25
                "H3",   // Spot 3(b) is dealt 3, has 11
                "ST",   // Spot 3(b) doubles on 11, stands on 21
                "S5",   // Spot 4 hits 14, stands on 19
                "D8",   // Spot 5 hits 12, stands on 20
                "H2",   // Spot 6 doubles on 10, stands on 12

                "H3",   // Dealer hits 16, stands on 19
            ],
            vec![10, -10 + 0, -10 + 20, 0, 10, -20]
        ),
        TestCase::new("Low soft totals aginst low up card",
            vec![
                "CA", "CA", "CA", "CA", "CA", "CA",     "D4",
                "C2", "C3", "C4", "C5", "C6", "C7",     "DT",

                "H5",   // Spot 1 hits soft 13, stands on soft 18
                "HA",   // Spot 2 hits soft 14, has soft 15
                "C6",   // Spot 2 hits soft 15, stands on soft 21
                "C7",   // Spot 3 doubles soft 15, stands on hard 12
                "S3",   // Spot 4 doubles soft 16, stands on soft 19
                "SJ",   // Spot 5 doubles soft 17, stands on hard 17
                "S8",   // Spot 6 doubles on soft 18, stands on hard 16

                "S4",   // Dealer hits 14, stands on 18
            ],
            vec![0, 10, -20, 20, -20, -20]
        ),
        TestCase::new("Low totals that cannot double",
            vec![
                "C2", "D3", "D5", "D4", "D6", "D5",     "S6",
                "C4", "D4", "D2", "D3", "D2", "D3",     "H5",

                "H3",   // Spot 1 hits 6, has 9
                "HA",   // Spot 1 hits 9, stands on soft 20
                "S3",   // Spot 2 hits 7, has 10
                "H6",   // Spot 2 hits 10, stands on 16
                "S4",   // Spot 3 hits 7, has 11
                "DK",   // Spot 3 hits 11, stands on 21
                "S2",   // Spot 4 hits 7, has 9
                "S9",   // Spot 4 hits 9, stands on 18
                "S2",   // Spot 5 hits 8, has 10
                "HQ",   // Spot 5 hits 10, stands on 20
                "S3",   // Spot 6 hits 8, has 11
                "D2",   // Spot 6 hits 11, stands on 13
                
                "S4",   // Dealer hits 11, has 15
                "S2",   // Dealer hits 15, stands on 17
            ],
            vec![10, -10, 10, 10, 10, -10]
        ),
        TestCase::new("Dealer shows a 9",
            vec![
                "H5", "C9", "DA", "SA", "CA", "H3",		"D9",
                "D9", "C2", "D8", "C4",	"CA", "H3",		"DT",

                "H3",	// Spot 1 hits 14, stands on 17
                "S2",	// Spot 2 doubles on 11, stands on 13
                        // Spot 3 stands on soft 19
                "SA",	// Spot 4 hits soft 15, has soft 16
                "S6",	// Spot 4 hits soft 16, has hard 12
                "S3",	// Spot 4 hits hard 12, has hard 15
                "H6",	// Spot 4 hits hard 15, stands on 21
                        // Spot 5 splits A's
                "D2",	// Spot 5.1 draws 2, has 13
                "D9",	// Spot 5.2 draws 9, has 20
                "S8",	// Spot 6 hits 6, has 14
                "D4",	// Spot 6 hits 14, stands on 18
            ],
            vec![-10, -20, 0, 10, -10 + 10, -10]
            // vec![-10, -20, 0, 10, 0, -10]
        ),
        TestCase::new("Dealer shows an A",
            vec![
                "H5", "HA", "D6", "D4", "D9", "HA",		"DA",
                "C8", "C3", "DA", "D4", "C9", "H2",		"D7",

                "S9", // Spot 1 hits 13, has 22
                "DJ", // Spot 2 hits soft 14, has hard 14
                "S5", // Spot 2 hits hard 14, has hard 19
                "HT", // Spot 3 hits soft 17, has hard 17
                "D3", // Spot 4 hits 8, has 11
                "S7", // Spot 4 hits 11, has 18
                        // Spot 5 stands on 18
                "D5", // Spot 6 hits soft 13, has soft 18
                "DK", // Spot 6 hits soft 18, has hard 18
            ],
            vec![-10, 10, -10, 0, 0, 0]
        ),
        TestCase::new("Dealer shows a 5",
            vec![
                "H5", "C9", "DA", "SA", "C9", "H3",		"D5",
                "D5", "C2", "D8", "C4",	"C9", "H3",		"DT",

                "H3",	// Spot 1 doubles on 10, stands on 13
                "S8",	// Spot 2 doubles on 11, stands on 19
                        // Spot 3 stands on soft 19
                "SA",	// Spot 4 hits doubles 15, has soft 16
                        // Spot 5 splits 9's
                "D4",	// Spot 5.1 draws 4, has 13
                "D9",	// Spot 5.2 draws 9, has 18
                        // Spot 5.2 splits 9's
                "D6",	// Spot 5.2.1 draws 6, has 15
                "DT",	// Spot 5.2.2 draws 10, has 19
                        // Spot 6 splits 3's
                "D2",	// Spot 6.1 draws 2, has 5
                "D3",	// Spot 6.1 draws 3, has 8
                "D4",	// Spot 6.1 draws 4, stands on 12
                "S7",	// Spot 6.2 draws 7, has 10
                "D4",	// Spot 6.2 doubles on 10, stands on 14

                "H7",	// Dealer hits 15, busts with 22
            ],
            vec![20, 20, 10, 20, 30, 30]
        ),
        TestCase::new("Pairs against a 3",
            vec![
                "H5", "HA", "D6", "D4", "D9", "H8",		"D3",
                "H5", "HA", "D6", "D4", "D9", "H8",		"DT",

                "S9", // Spot 1 doubles on 10, has 19
                        // Spot 2 splits A's
                "D2", // Spot 2.1 draws 2, has 11
                "D9", // Spot 2.2 draws 9, has soft 20
                        // Spot 3 splits 6's
                "S8", // Spot 3.1 draws 8, stands on 14
                "SK", // Spot 3.2 draws 10, has 16
                "D5", // Spot 4 hits on 8, stands on 13
                        // Spot 5 splits 9's
                "D5", // Spot 5.1 draws 5, has 14
                "D6", // Spot 5.2 draws 6, has 15
                        // Spot 6 splits 8's
                "H4", // Spot 6.1 draws 4, has 12
                "H4", // Spot 6.1 draws 4, has 16
                "CA", // Spot 6.2 draws A, has soft 19

                "CT", // Dealer hits 13, busts with 23
            ],
            vec![20, 20, 20, 10, 20, 20]
        )
    ];

    let mut game = Blackjack::new(BlackjackConfig::standard());
    let mut expectations: Vec<f32> = vec![0.; 6];

    for _ in 0..6 {
        game.add_player(Box::new(BasicPolicy::new()));
    }

    for (case_idx, case) in cases.iter().enumerate() {
        let mut shorts: Vec<String> = case.deck.clone();
        // shorts.append(vec!["DK".to_string(); 15].as_mut());
        let mut cards: Vec<Card> = shorts.iter().map(|s| Card::from_short(s).unwrap()).collect();
        cards.reverse();
        game.deck = Deck::from_cards(cards);

        for (player_idx, player_bank_delta) in case.results.iter().enumerate() {
            expectations[player_idx] += *player_bank_delta as f32;
        }

        println!("Starting case {}: {}", case_idx+1, case.title);
        
        let mut game_finished = false;
        while !game_finished {
            match game.next() {
                Ok(_) => {
                    match game.state {
                        BlackjackState::Cleanup => {
                            println!("Dealer has {}", game.dealer.pretty());
                            for (player_idx, player) in game.players.iter().enumerate() {
                                for (spot_idx, spot) in player.state.spots.iter().enumerate() {
                                    println!("Player {}.{} has {}", player_idx+1, spot_idx+1, spot.cards.pretty());
                                }
                            }
                        }
                        BlackjackState::Prebetting => {
                            game_finished = true;
                            for (player_idx, player) in game.players.iter().enumerate() {
                                assert_eq!(player.state.get_bank(), expectations[player_idx], "Case '{}' ({}) player {} bank", case.title, case_idx+1, player_idx+1);
                            }

                            println!("Case {} passed", case_idx+1);
                        }
                        BlackjackState::Betting => (),
                        BlackjackState::Dealing => (),
                        BlackjackState::Action => (),
                        BlackjackState::Results => (),
                    };
                },
                Err(_err) => (),
            }
        }
    }

}