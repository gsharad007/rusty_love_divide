use super::deck::Deck;
use super::tile::Tile;
use itertools::iproduct;

// trait DeckGenerator<CardType> {
//     fn generate_all_permutations_of_tiles() -> Deck<CardType>;
// }

const TILE_ROOT: Tile = Tile::new([0, 0, 0, 0]);

/// Generates All Permutations of 4 Edge Type Tiles
///
/// # Examples
///
/// ```
/// let deck = generate_all_permutations_of_tiles();
///
/// assert_eq!(71, deck.len(), "There should have been 71 (70 unique square tiles + 1 root tile)s generated in the game with 4 edge types.");
///
/// assert!(deck.contains(&Tile::new([0, 0, 0, 0])));
/// assert!(deck.contains(&Tile::new([1, 1, 1, 1])));
/// assert!(deck.contains(&Tile::new([2, 2, 2, 2])));
/// assert!(deck.contains(&Tile::new([3, 3, 3, 3])));
/// assert!(deck.contains(&Tile::new([4, 4, 4, 4])));
///
/// assert!(deck.contains(&Tile::new([1, 2, 3, 4])));
/// assert!(deck.contains(&Tile::new([2, 1, 4, 3])));
/// assert!(deck.contains(&Tile::new([4, 3, 2, 1])));
///
/// assert!(!deck.contains(&Tile::new([0, 1, 2, 3])));
/// assert!(!deck.contains(&Tile::new([2, 1, 0, 3])));
/// assert!(!deck.contains(&Tile::new([4, 3, 2, 0])));
/// ```
pub fn generate_deck_with_all_permutations() -> Deck<Tile> {
    const TILE_COUNT: usize = 70 + 1; // 70 Player Tiles + 1 Home Tile
    let mut tiles = Vec::<Tile>::with_capacity(TILE_COUNT);
    for (e1, e2, e3, e4) in iproduct!(1..=4, 1..=4, 1..=4, 1..=4) {
        let tile = Tile::new([e1, e2, e3, e4]);
        if !tiles.contains(&tile) {
            tiles.push(tile);
        }
    }
    tiles.push(TILE_ROOT);
    tiles
}

#[cfg(test)]
mod tests_generating_permutations {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_generate_all_permutations_of_square_tiles_with_4_edge_types() {
        let deck = generate_deck_with_all_permutations();
        assert_eq!(71, deck.len(), "There should have been 71 (70 unique square tiles + 1 root tile)s generated in the deck with 4 edge types.");

        assert!(deck.contains(&Tile::new([0, 0, 0, 0])));

        for (e1, e2, e3, e4) in iproduct!(1..=4, 1..=4, 1..=4, 1..=4) {
            let tile = Tile::new([e1, e2, e3, e4]);
            assert!(
                deck.contains(&tile),
                "Deck should have contained a tile equivalent to {:?}",
                tile
            );
        }
    }

    #[test]
    fn test_generate_all_permutations_of_square_tiles_is_unique() {
        let deck = generate_deck_with_all_permutations();
        let deck_unique = deck.clone().into_iter().unique().collect_vec();
        assert_eq!(deck.len(), deck_unique.len(), "There should be 71 (70 unique square tiles + 1 root tile)s in the deck with 4 edge types. Seems not all are unique.");
        itertools::assert_equal(&deck, &deck_unique);
    }

    // #[test]
    // fn test_generate_all_permutations_of_square_tiles_is_ascending_value() {
    //     let deck = generate_all_permutations_of_tiles();
    //     let mut deck_sorted = deck.clone();
    //     deck_sorted.sort_by_key(|a| a.calculate_value());
    //     assert_eq!(deck.len(), deck_sorted.len(), "There should be 71 (70 unique square tiles + 1 root tile)s in the deck with 4 edge types. Seems not all are unique.");

    //     itertools::assert_equal(&deck[..deck.len() - 1], &deck_sorted[1..]);
    // }
}
