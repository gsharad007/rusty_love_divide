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
#[derive(Clone, Eq, Hash, Debug)]
pub struct Tile {
    edges: [u8; 4],
}

impl Tile {
    pub fn new(edges: [u8; 4]) -> Tile {
        Tile { edges }
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
            self.edges
        );
        Tile::new([self.edges[3], self.edges[0], self.edges[1], self.edges[2]])
    }
    fn rotate_counter_clockwise(&self) -> Self {
        assert_eq!(
            self.edges.len(),
            4,
            "This tile {:?} does not have 4 edges. INCONCEIVABLE!",
            self.edges
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
            rhs = rhs.rotate_clockwise();
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

        assert_ne!(Tile::new([1, 1, 1, 1]), Tile::new([2, 2, 2, 2]));
        assert_ne!(Tile::new([1, 1, 1, 1]), Tile::new([1, 2, 3, 4]));

        assert_ne!(Tile::new([1, 2, 3, 4]), Tile::new([4, 3, 2, 1]));
        assert_ne!(Tile::new([1, 2, 3, 4]), Tile::new([2, 1, 3, 4]));
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
        let deck_unique = deck.clone().into_iter().unique().collect_vec();

        assert_eq!(71, deck.len(), "There should have been 71 (70 unique square tiles + 1 root tile)s generated in the game with 4 edge types.");
        assert_eq!(deck.len(), deck_unique.len(), "There should be 71 (70 unique square tiles + 1 root tile)s in the game with 4 edge types. Seems not all are unique.");
        itertools::assert_equal(&deck, &deck_unique);

        assert!(deck.contains(&Tile::new([0, 0, 0, 0])));
        assert!(deck.contains(&Tile::new([1, 1, 1, 1])));
        assert!(deck.contains(&Tile::new([2, 2, 2, 2])));
        assert!(deck.contains(&Tile::new([3, 3, 3, 3])));
        assert!(deck.contains(&Tile::new([4, 4, 4, 4])));

        assert!(deck.contains(&Tile::new([1, 2, 3, 4])));
        assert!(deck.contains(&Tile::new([2, 1, 4, 3])));
        assert!(deck.contains(&Tile::new([4, 3, 2, 1])));

        assert!(!deck.contains(&Tile::new([0, 1, 2, 3])));
        assert!(!deck.contains(&Tile::new([2, 1, 0, 3])));
        assert!(!deck.contains(&Tile::new([4, 3, 2, 0])));
    }
}
