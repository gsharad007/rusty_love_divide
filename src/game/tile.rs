/// A Sqaure Tile with 4 edges
///
/// # Examples
///
/// ```
/// let root = Tile {
///     edges: [0, 0, 0, 0],
/// };
/// assert_eq!(4, root.edges.len());
/// assert_eq!(0, root.edges[0]);
///
/// let user_tile = Tile {
///     edges: [1, 2, 3, 4],
/// };
/// assert_eq!(1, root.edges[0]);
/// assert_eq!(2, root.edges[1]);
/// assert_eq!(3, root.edges[2]);
/// assert_eq!(4, root.edges[3]);
/// ```
#[derive(Clone, Eq, Debug)]
pub struct Tile {
    edges: [u8; 4],
}

impl Tile {
    pub fn new(edges: [u8; 4]) -> Tile {
        Tile { edges }
    }
}

#[cfg(test)]
mod tests_constructible_construct {
    use super::*;

    #[test]
    fn test_struct_construtible() {
        let root = Tile {
            edges: [0, 0, 0, 0],
        };
        assert_eq!(4, root.edges.len());
        assert_eq!(0, root.edges[0]);

        let user_tile = Tile {
            edges: [1, 2, 3, 4],
        };
        assert_eq!(4, user_tile.edges.len());
        assert_eq!(1, user_tile.edges[0]);
        assert_eq!(2, user_tile.edges[1]);
        assert_eq!(3, user_tile.edges[2]);
        assert_eq!(4, user_tile.edges[3]);
    }

    #[test]
    fn test_new_construtible() {
        let root = Tile::new([0, 0, 0, 0]);
        assert_eq!(4, root.edges.len());
        assert_eq!(0, root.edges[0]);

        let user_tile = Tile::new([1, 2, 3, 4]);
        assert_eq!(4, user_tile.edges.len());
        assert_eq!(1, user_tile.edges[0]);
        assert_eq!(2, user_tile.edges[1]);
        assert_eq!(3, user_tile.edges[2]);
        assert_eq!(4, user_tile.edges[3]);
    }
}

trait Rotatable {
    #[must_use]
    fn rotate_clockwise(&self) -> Self;

    #[must_use]
    fn rotate_counter_clockwise(&self) -> Self;
}

impl Rotatable for Tile {
    fn rotate_clockwise(&self) -> Self {
        assert_eq!(
            self.edges.len(),
            4,
            "This tile {:?} does not have 4 edges. INCONCEIVABLE!",
            self
        );
        Tile::new([self.edges[3], self.edges[0], self.edges[1], self.edges[2]])
    }
    fn rotate_counter_clockwise(&self) -> Self {
        assert_eq!(
            self.edges.len(),
            4,
            "This tile {:?} does not have 4 edges. INCONCEIVABLE!",
            self
        );
        Tile::new([self.edges[1], self.edges[2], self.edges[3], self.edges[0]])
    }
}

#[cfg(test)]
mod tests_rotatable {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_rotate_clockwise() {
        assert_eq!(Tile::new([1, 1, 1, 1]).edges, Tile::new([1, 1, 1, 1]).rotate_clockwise().edges);

        assert_eq!(Tile::new([1, 1, 1, 2]).edges, Tile::new([1, 1, 1, 2]).edges);
        assert_eq!(Tile::new([2, 1, 1, 1]).edges, Tile::new([1, 1, 1, 2]).rotate_clockwise().edges);
        assert_eq!(Tile::new([1, 2, 1, 1]).edges, Tile::new([1, 1, 1, 2]).rotate_clockwise().rotate_clockwise().edges);
        assert_eq!(Tile::new([1, 1, 2, 1]).edges, Tile::new([1, 1, 1, 2]).rotate_clockwise().rotate_clockwise().rotate_clockwise().edges);

        assert_eq!(Tile::new([4, 1, 2, 3]).edges, Tile::new([1, 2, 3, 4]).rotate_clockwise().edges);
        assert_eq!(Tile::new([3, 4, 1, 2]).edges, Tile::new([1, 2, 3, 4]).rotate_clockwise().rotate_clockwise().edges);
        assert_eq!(Tile::new([2, 3, 4, 1]).edges, Tile::new([1, 2, 3, 4]).rotate_clockwise().rotate_clockwise().rotate_clockwise().edges);

        assert_ne!(Tile::new([1, 1, 1, 1]).edges, Tile::new([2, 2, 2, 2]).rotate_clockwise().edges);
        assert_ne!(Tile::new([1, 1, 1, 1]).edges, Tile::new([1, 2, 3, 4]).rotate_clockwise().edges);

        assert_ne!(Tile::new([4, 3, 2, 1]).edges, Tile::new([1, 2, 3, 4]).rotate_clockwise().edges);
        assert_ne!(Tile::new([2, 1, 3, 4]).edges, Tile::new([1, 2, 3, 4]).rotate_clockwise().edges);
    }

    #[test]
    #[rustfmt::skip]
    fn test_rotate_counter_clockwise() {
        assert_eq!(Tile::new([1, 1, 1, 1]).edges, Tile::new([1, 1, 1, 1]).rotate_counter_clockwise().edges);

        assert_eq!(Tile::new([1, 1, 1, 2]).edges, Tile::new([1, 1, 1, 2]).edges);
        assert_eq!(Tile::new([1, 1, 2, 1]).edges, Tile::new([1, 1, 1, 2]).rotate_counter_clockwise().edges);
        assert_eq!(Tile::new([1, 2, 1, 1]).edges, Tile::new([1, 1, 1, 2]).rotate_counter_clockwise().rotate_counter_clockwise().edges);
        assert_eq!(Tile::new([2, 1, 1, 1]).edges, Tile::new([1, 1, 1, 2]).rotate_counter_clockwise().rotate_counter_clockwise().rotate_counter_clockwise().edges);

        assert_eq!(Tile::new([2, 3, 4, 1]).edges, Tile::new([1, 2, 3, 4]).rotate_counter_clockwise().edges);
        assert_eq!(Tile::new([3, 4, 1, 2]).edges, Tile::new([1, 2, 3, 4]).rotate_counter_clockwise().rotate_counter_clockwise().edges);
        assert_eq!(Tile::new([4, 1, 2, 3]).edges, Tile::new([1, 2, 3, 4]).rotate_counter_clockwise().rotate_counter_clockwise().rotate_counter_clockwise().edges);

        assert_ne!(Tile::new([1, 1, 1, 1]).edges, Tile::new([2, 2, 2, 2]).rotate_counter_clockwise().edges);
        assert_ne!(Tile::new([1, 1, 1, 1]).edges, Tile::new([1, 2, 3, 4]).rotate_counter_clockwise().edges);

        assert_ne!(Tile::new([4, 3, 2, 1]).edges, Tile::new([1, 2, 3, 4]).rotate_counter_clockwise().edges);
        assert_ne!(Tile::new([2, 1, 3, 4]).edges, Tile::new([1, 2, 3, 4]).rotate_counter_clockwise().edges);
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        let mut rhs = other.clone();
        for _r in 0..4 {
            if self.edges == rhs.edges {
                return true;
            }
            rhs = rhs.rotate_counter_clockwise();
        }
        false
    }
}

#[cfg(test)]
mod tests_equality {
    use super::*;
    #[test]
    fn test_equality_on_tile() {
        assert_eq!(Tile::new([1, 1, 1, 1]), Tile::new([1, 1, 1, 1]));
        assert_eq!(Tile::new([1, 1, 1, 2]), Tile::new([2, 1, 1, 1]));
        assert_eq!(Tile::new([1, 1, 1, 2]), Tile::new([1, 2, 1, 1]));
        assert_eq!(Tile::new([1, 1, 1, 2]), Tile::new([1, 1, 2, 1]));

        assert_eq!(Tile::new([1, 2, 3, 4]), Tile::new([4, 1, 2, 3]));
        assert_eq!(Tile::new([1, 2, 3, 4]), Tile::new([3, 4, 1, 2]));
        assert_eq!(Tile::new([1, 2, 3, 4]), Tile::new([2, 3, 4, 1]));

        assert_eq!(Tile::new([2, 2, 3, 4]), Tile::new([4, 2, 2, 3]));
        assert_eq!(Tile::new([2, 2, 3, 4]), Tile::new([3, 4, 2, 2]));
        assert_eq!(Tile::new([2, 2, 3, 4]), Tile::new([2, 3, 4, 2]));

        assert_ne!(Tile::new([1, 1, 1, 1]), Tile::new([2, 2, 2, 2]));
        assert_ne!(Tile::new([1, 1, 1, 1]), Tile::new([1, 2, 3, 4]));

        assert_ne!(Tile::new([1, 2, 3, 4]), Tile::new([4, 3, 2, 1]));
        assert_ne!(Tile::new([1, 2, 3, 4]), Tile::new([2, 1, 3, 4]));
    }
}

impl Tile {
    fn calculate_value(&self) -> u32 {
        self.edges
            .iter()
            .rev()
            .enumerate()
            .fold(0, |result, tuple| {
                result + ((*tuple.1) as u32 * 10_u32.pow(tuple.0 as u32))
            })
    }

    fn find_starting_edge(&self) -> usize {
        let mut min_view = u32::MAX;
        let mut min_view_index = 0;
        let mut tmp = self.clone();
        for idx in 0..4 {
            let view = tmp.calculate_value();
            if view < min_view {
                min_view = view;
                min_view_index = idx;
            }
            tmp = tmp.rotate_counter_clockwise();
        }
        min_view_index
    }
}

#[cfg(test)]
mod tests_derotation_helpers {
    use super::*;

    #[test]
    fn test_calculate_value() {
        assert_eq!(1111, Tile::new([1, 1, 1, 1]).calculate_value());
        assert_eq!(2111, Tile::new([2, 1, 1, 1]).calculate_value());
        assert_eq!(1211, Tile::new([1, 2, 1, 1]).calculate_value());
        assert_eq!(1121, Tile::new([1, 1, 2, 1]).calculate_value());

        assert_eq!(4123, Tile::new([4, 1, 2, 3]).calculate_value());
        assert_eq!(3412, Tile::new([3, 4, 1, 2]).calculate_value());
        assert_eq!(2341, Tile::new([2, 3, 4, 1]).calculate_value());

        assert_eq!(4223, Tile::new([4, 2, 2, 3]).calculate_value());
        assert_eq!(3422, Tile::new([3, 4, 2, 2]).calculate_value());
        assert_eq!(2342, Tile::new([2, 3, 4, 2]).calculate_value());

        assert_eq!(2222, Tile::new([2, 2, 2, 2]).calculate_value());
        assert_eq!(1234, Tile::new([1, 2, 3, 4]).calculate_value());

        assert_eq!(4321, Tile::new([4, 3, 2, 1]).calculate_value());
        assert_eq!(2134, Tile::new([2, 1, 3, 4]).calculate_value());
    }

    #[test]
    fn test_find_starting_edge() {
        assert_eq!(0, Tile::new([1, 1, 1, 1]).find_starting_edge());
        assert_eq!(1, Tile::new([2, 1, 1, 1]).find_starting_edge());
        assert_eq!(2, Tile::new([1, 2, 1, 1]).find_starting_edge());
        assert_eq!(3, Tile::new([1, 1, 2, 1]).find_starting_edge());

        assert_eq!(1, Tile::new([4, 1, 2, 3]).find_starting_edge());
        assert_eq!(2, Tile::new([3, 4, 1, 2]).find_starting_edge());
        assert_eq!(3, Tile::new([2, 3, 4, 1]).find_starting_edge());

        assert_eq!(1, Tile::new([4, 2, 2, 3]).find_starting_edge());
        assert_eq!(2, Tile::new([3, 4, 2, 2]).find_starting_edge());
        assert_eq!(3, Tile::new([2, 3, 4, 2]).find_starting_edge());

        assert_eq!(0, Tile::new([2, 2, 2, 2]).find_starting_edge());
        assert_eq!(0, Tile::new([1, 2, 3, 4]).find_starting_edge());

        assert_eq!(3, Tile::new([4, 3, 2, 1]).find_starting_edge());
        assert_eq!(1, Tile::new([2, 1, 3, 4]).find_starting_edge());
    }
}

use std::hash::{Hash, Hasher};
impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let min_value_index = self.find_starting_edge();

        let mut idx = min_value_index;
        for _ in 0..4 {
            self.edges[idx].hash(state);
            idx = (idx + 1) % 4;
        }
    }
}

#[cfg(test)]
mod tests_hash {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    macro_rules! assert_hash_eq {
        ($left:expr, $right:expr $(,)?) => {{
            assert_eq!(
                calculate_hash(&$left),
                calculate_hash(&$right),
                "{:?} == {:?} => calculate_value() {:?} == {:?} => find_starting_edge() {:?} == {:?}",
                &$left,
                &$right,
                &$left.calculate_value(),
                &$right.calculate_value(),
                &$left.find_starting_edge(),
                &$right.find_starting_edge(),
            );
        }};
    }

    macro_rules! assert_hash_ne {
        ($left:expr, $right:expr $(,)?) => {{
            assert_ne!(
                calculate_hash(&$left),
                calculate_hash(&$right),
                "{:?} == {:?} => calculate_value() {:?} == {:?} => find_starting_edge() {:?} == {:?}",
                &$left,
                &$right,
                &$left.calculate_value(),
                &$right.calculate_value(),
                &$left.find_starting_edge(),
                &$right.find_starting_edge(),
            );
        }};
    }

    #[test]
    fn test_hash_equality_on_tile() {
        assert_hash_eq!(Tile::new([1, 1, 1, 1]), Tile::new([1, 1, 1, 1]));
        assert_hash_eq!(Tile::new([1, 1, 1, 2]), Tile::new([2, 1, 1, 1]));
        assert_hash_eq!(Tile::new([1, 1, 1, 2]), Tile::new([2, 1, 1, 1]));
        assert_hash_eq!(Tile::new([1, 1, 1, 2]), Tile::new([1, 2, 1, 1]));
        assert_hash_eq!(Tile::new([1, 1, 1, 1]), Tile::new([1, 1, 1, 1]));
        assert_hash_eq!(Tile::new([1, 1, 1, 2]), Tile::new([1, 1, 2, 1]));

        assert_hash_eq!(Tile::new([1, 2, 3, 4]), Tile::new([4, 1, 2, 3]));
        assert_hash_eq!(Tile::new([1, 2, 3, 4]), Tile::new([3, 4, 1, 2]));
        assert_hash_eq!(Tile::new([1, 2, 3, 4]), Tile::new([2, 3, 4, 1]));

        assert_hash_eq!(Tile::new([2, 2, 3, 4]), Tile::new([4, 2, 2, 3]));
        assert_hash_eq!(Tile::new([2, 2, 3, 4]), Tile::new([3, 4, 2, 2]));
        assert_hash_eq!(Tile::new([2, 2, 3, 4]), Tile::new([2, 3, 4, 2]));

        assert_hash_ne!(Tile::new([1, 1, 1, 1]), Tile::new([2, 2, 2, 2]));
        assert_hash_ne!(Tile::new([1, 1, 1, 1]), Tile::new([1, 2, 3, 4]));

        assert_hash_ne!(Tile::new([1, 2, 3, 4]), Tile::new([4, 3, 2, 1]));
        assert_hash_ne!(Tile::new([1, 2, 3, 4]), Tile::new([2, 1, 3, 4]));
    }
}

const TILE_ROOT: Tile = Tile {
    edges: [0, 0, 0, 0],
};

use itertools::iproduct;

/// Generates All Permutations of 4 Edge Type Tiles
///
/// # Examples
///
/// ```
/// let deck = Tile::generate_all_permutations_of_tiles();
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
impl Tile {
    #[allow(dead_code)]
    pub fn generate_all_permutations_of_tiles() -> Vec<Tile> {
        const TILE_COUNT: usize = 70 + 1; // 70 Player Tiles + 1 Home Tile
        let mut tiles = Vec::with_capacity(TILE_COUNT);
        for (e1, e2, e3, e4) in iproduct!(1..=4, 1..=4, 1..=4, 1..=4) {
            let tile = Tile::new([e1, e2, e3, e4]);
            if !tiles.contains(&tile) {
                tiles.push(tile);
            }
        }
        tiles.push(TILE_ROOT);
        tiles
    }
}

#[cfg(test)]
mod tests_generating_permutations {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_generate_all_permutations_of_square_tiles_with_4_edge_types() {
        let deck = Tile::generate_all_permutations_of_tiles();
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
        let deck = Tile::generate_all_permutations_of_tiles();
        let deck_unique = deck.clone().into_iter().unique().collect_vec();
        assert_eq!(deck.len(), deck_unique.len(), "There should be 71 (70 unique square tiles + 1 root tile)s in the deck with 4 edge types. Seems not all are unique.");
        itertools::assert_equal(&deck, &deck_unique);
    }

    #[test]
    fn test_generate_all_permutations_of_square_tiles_is_ascending_value() {
        let deck = Tile::generate_all_permutations_of_tiles();
        let mut deck_sorted = deck.clone();
        deck_sorted.sort_by_key(|a| a.calculate_value());
        assert_eq!(deck.len(), deck_sorted.len(), "There should be 71 (70 unique square tiles + 1 root tile)s in the deck with 4 edge types. Seems not all are unique.");

        itertools::assert_equal(&deck[..deck.len() - 1], &deck_sorted[1..]);
    }
}
