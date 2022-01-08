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
    pub const fn new(edges: [u8; 4]) -> Tile {
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
    pub fn calculate_value(&self) -> u32 {
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
