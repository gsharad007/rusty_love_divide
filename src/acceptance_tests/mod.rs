use super::game::deck::Deck;
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
        use crate::game::deck_generator::generate_deck_with_all_permutations;
        use anyhow::Result;
        use itertools::Itertools;

        #[test]
        fn then_setup_game() -> Result<()> {
            let mut game_session = GameSession {
                game_deck: generate_deck_with_all_permutations(),
                play_deck: Deck::new(),
            };
            assert_eq!(71, game_session.game_deck.len(), "There should have been 71 (70 unique square tiles + 1 root tile)s generated in the game with 4 edge types.");

            game_session.play_deck = game_session.game_deck.iter().filter(|_| true).collect_vec();
            assert_eq!(71, game_session.play_deck.len(), "There should have been 71 (70 unique square tiles + 1 root tile)s generated in the game with 4 edge types.");

            Ok(())
        }
    }
}
