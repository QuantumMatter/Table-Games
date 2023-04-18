use table_games::blackjack::{Blackjack, BlackjackConfig, Ploppy};

fn main() {
    println!("Hello, world!");

    let config = BlackjackConfig::standard();
    let mut game = Blackjack::new(config);

    game.add_player(Box::new(Ploppy {}));

    loop {
        match game.next() {
            Ok(_state) => {
                println!("State: {:?}", game.get_state());
            },
            Err(_err) => {
                break;
            }
        }
    }
}
