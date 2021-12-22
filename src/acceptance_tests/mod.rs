use super::game::tile::Tile;

#[derive(Debug)]
struct GameSession {
    game_deck: Deck,
}

type Deck = Vec<Tile>;


#[cfg(test)]
mod given_square_grid_and_tiles {
    mod when_starting_new_game {
        use super::super::*;
        use anyhow::Result;

        #[test]
        fn then_setup_game() -> Result<()> {
            let game_session = GameSession {
                game_deck: Tile::generate_all_permutations_of_tiles(),
            };

            assert_eq!(71, game_session.game_deck.len(), "There should have been 71 (70 unique square tiles + 1 root tile)s generated in the game with 4 edge types.");

            Ok(())
        }
    }
}
