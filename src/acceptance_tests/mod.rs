use super::game::deck::Deck;
// use super::game::deck_generator::DeckGenerator;
use super::game::tile::Tile;

#[derive(Debug)]
struct GameSession<'a> {
    game_deck: Deck<Tile>,
    play_deck: Deck<&'a Tile>,
}

#[cfg(test)]
mod given_square_grid_and_tiles {
    mod when_starting_new_game {
        use super::super::*;
        use crate::game::deck_generator::DeckGenerator;
        use anyhow::Result;
        use itertools::Itertools;

        #[test]
        fn test_game_session_debug() {
            let game_session = GameSession {
                game_deck: Deck::new(),
                play_deck: Deck::new(),
            };
            assert_eq!(
                "GameSession { game_deck: [], play_deck: [] }",
                format!("{game_session:?}")
            );
        }

        #[test]
        fn then_setup_game() -> Result<()> {
            let mut game_session = GameSession {
                game_deck: Deck::generate_all_permutations_and_root(),
                play_deck: Deck::new(),
            };
            assert_eq!(71, game_session.game_deck.len(), "There should have been 71 (70 unique square tiles + 1 root tile)s generated in the game with 4 edge types.");

            game_session.play_deck = game_session.game_deck.iter().filter(|_| true).collect_vec();
            assert_eq!(71, game_session.play_deck.len(), "There should have been 71 (70 unique square tiles + 1 root tile)s generated in the game with 4 edge types.");

            Ok(())
        }
    }
}
