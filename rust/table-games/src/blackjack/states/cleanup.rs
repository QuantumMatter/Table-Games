use crate::{blackjack::{Blackjack, BlackjackState, BlackjackStateHandler, BlackjackError}, common::Deck};

pub struct CleanupStateHandler {}
impl BlackjackStateHandler for CleanupStateHandler {
    fn execute(game: &mut Blackjack) -> Result<BlackjackState, BlackjackError> {

        game.dealer.clear();

        for player in game.players.iter_mut() {
            player.state.spots.clear();
        }

        let remaining_cards = game.deck.len();
        let after_cut_card = ((52 * game.config.deck_count) as f32) * (1.0 - game.config.pen);
        let after_cut_card = after_cut_card as usize;

        if remaining_cards < after_cut_card {
            game.deck = Deck::new();
            for _ in 1..game.config.deck_count {
                game.deck += Deck::new();
            }
            game.deck.shuffle();
            game.deck.draw();
        }

        Ok(BlackjackState::Prebetting)
    }
}