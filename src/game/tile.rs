use itertools::iproduct;
use std::ops::RangeInclusive;

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
    pub const fn from_tuple(tuple: (u8, u8, u8, u8)) -> Tile {
        Tile::new([tuple.0, tuple.1, tuple.2, tuple.3])
    }

    pub fn all_permutations(range: RangeInclusive<u8>) -> impl Iterator<Item = (u8, u8, u8, u8)> {
        iproduct!(range.clone(), range.clone(), range.clone(), range)
    }
}

#[cfg(test)]
mod tests_constructible_construct {
    use super::*;
    extern crate test;
    use core::hint::black_box;
    use test::Bencher;

    #[test]
    fn test_struct_constructible() {
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
    fn test_new_constructible() {
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

    #[bench]
    fn bench_struct_constructible(b: &mut Bencher) {
        let mut _t = Tile {
            edges: [0, 0, 0, 0],
        };
        b.iter(|| {
            Tile::all_permutations(1..=4).for_each(|edge| {
                _t = Tile {
                    edges: [edge.0, edge.1, edge.2, edge.3],
                };
            });
        });
    }

    #[bench]
    fn bench_new_constructible_for(b: &mut Bencher) {
        let mut _t = Tile::new([0, 0, 0, 0]);
        b.iter(|| {
            for (e1, e2, e3, e4) in Tile::all_permutations(1..=4) {
                _t = black_box(Tile::new([e1, e2, e3, e4]));
            }
        });
    }

    #[bench]
    fn bench_new_constructible(b: &mut Bencher) {
        let mut _t = Tile::from_tuple((0, 0, 0, 0));
        b.iter(|| {
            Tile::all_permutations(1..=4).for_each(|edge| {
                _t = Tile::new([edge.0, edge.1, edge.2, edge.3]);
            });
        });
    }

    #[bench]
    fn bench_new_constructible_from_tuple(b: &mut Bencher) {
        let mut _t = Tile::from_tuple((0, 0, 0, 0));
        b.iter(|| {
            Tile::all_permutations(1..=4).for_each(|edge| {
                _t = Tile::from_tuple(edge);
            });
        });
    }
}

trait Rotatable {
    #[must_use]
    fn rotate_counter_clockwise_with_offset(&self, offset: i8) -> Self;

    #[must_use]
    fn rotate_clockwise(&self) -> Self
    where
        Self: std::marker::Sized,
    {
        self.rotate_counter_clockwise_with_offset(-1)
    }

    #[must_use]
    fn rotate_counter_clockwise(&self) -> Self
    where
        Self: std::marker::Sized,
    {
        self.rotate_counter_clockwise_with_offset(1)
    }
}

impl Rotatable for Tile {
    #[allow(clippy::identity_op)]
    fn rotate_counter_clockwise_with_offset(&self, offset: i8) -> Self {
        assert_eq!(
            self.edges.len(),
            4,
            "This tile {:?} does not have 4 edges. INCONCEIVABLE!",
            self
        );
        Tile::new([
            self.edges[((0_i8 + offset).rem_euclid(4)) as usize],
            self.edges[((1_i8 + offset).rem_euclid(4)) as usize],
            self.edges[((2_i8 + offset).rem_euclid(4)) as usize],
            self.edges[((3_i8 + offset).rem_euclid(4)) as usize],
        ])
    }
}

#[cfg(test)]
mod tests_rotatable {
    use super::*;
    extern crate test;
    use core::hint::black_box;
    use test::Bencher;

    #[test]
    fn test_mod_offset() {
        assert_eq!(0, (0_i8 + (0)).rem_euclid(4));
        assert_eq!(3, (0_i8 + (-1)).rem_euclid(4));
        assert_eq!(2, (0_i8 + (-2)).rem_euclid(4));
        assert_eq!(1, (0_i8 + (-3)).rem_euclid(4));
        assert_eq!(0, (0_i8 + (-4)).rem_euclid(4));

        assert_eq!(0, (0_i8 + (0)).rem_euclid(4));
        assert_eq!(1, (0_i8 + (1)).rem_euclid(4));
        assert_eq!(2, (0_i8 + (2)).rem_euclid(4));
        assert_eq!(3, (0_i8 + (3)).rem_euclid(4));
        assert_eq!(0, (0_i8 + (4)).rem_euclid(4));
    }

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

    #[bench]
    fn bench_rotate_clockwise(b: &mut Bencher) {
        let mut t = Tile::new([1, 2, 3, 4]);
        b.iter(black_box(|| {
            (0..10).for_each(|_| {
                t = t.rotate_clockwise();
            });
        }));
    }

    #[bench]
    fn bench_rotate_counter_clockwise(b: &mut Bencher) {
        let mut t = Tile::new([1, 2, 3, 4]);
        b.iter(black_box(|| {
            (0..10).for_each(|_| {
                t = t.rotate_counter_clockwise();
            })
        }));
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        (0..4).any(|i| self.edges == other.rotate_counter_clockwise_with_offset(i).edges)
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

    fn find_starting_edge(&self) -> u8 {
        self.find_starting_edge_v2()
    }
}

impl Tile {
    // V1 Second (989 ns/iter (+/- 12))
    #[cfg(test)]
    fn find_starting_edge_v1(&self) -> u8 {
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

    // V2 Fastest (359 ns/iter (+/- 5) --OR-- 1,132 ns/iter (+/- 23))
    // First faster number is as a trait where as the second is as an external function
    // Yet seems to slow down dependent functions
    fn find_starting_edge_v2(&self) -> u8 {
        (0..4)
            .min_by_key(|i| {
                self.rotate_counter_clockwise_with_offset(*i as i8)
                    .calculate_value()
            })
            .unwrap_or(0)
    }

    // V3 (1,124 ns/iter (+/- 13))
    #[cfg(test)]
    fn find_starting_edge_v3(&self) -> u8 {
        let mut min_value = u32::MAX;
        let mut min_value_index = 0;
        (0..4).for_each(|i| {
            let value = self
                .rotate_counter_clockwise_with_offset(i as i8)
                .calculate_value();
            if value < min_value {
                min_value = value;
                min_value_index = i;
            }
        });
        min_value_index
    }
}

#[cfg(test)]
mod tests_derotation_helpers {
    use super::*;
    extern crate test;
    use core::hint::black_box;
    use test::Bencher;

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

    macro_rules! assert_find_starting_edge_eq_all_versions {
        ($left:expr, $right:expr $(,)?) => {{
            assert_eq!(
                $left,
                $right.find_starting_edge(),
                "{:?} == {:?} => calculate_value() {:?} => find_starting_edge() {:?}",
                &$left,
                &$right,
                $right.calculate_value(),
                $right.find_starting_edge(),
            );
            assert_eq!(
                $right.find_starting_edge(),
                $right.find_starting_edge_v1(),
                "{:?} == {:?} => calculate_value() {:?} == {:?} => find_starting_edge() {:?} == find_starting_edge_v1() {:?}",
                &$right,
                &$right,
                $right.calculate_value(),
                $right.calculate_value(),
                $right.find_starting_edge(),
                $right.find_starting_edge_v1(),
            );
            assert_eq!(
                $right.find_starting_edge(),
                $right.find_starting_edge_v2(),
                "{:?} == {:?} => calculate_value() {:?} == {:?} => find_starting_edge() {:?} == find_starting_edge_v2() {:?}",
                &$right,
                &$right,
                $right.calculate_value(),
                $right.calculate_value(),
                $right.find_starting_edge(),
                $right.find_starting_edge_v2(),
            );
            assert_eq!(
                $right.find_starting_edge(),
                $right.find_starting_edge_v3(),
                "{:?} == {:?} => calculate_value() {:?} == {:?} => find_starting_edge() {:?} == find_starting_edge_v3() {:?}",
                &$right,
                &$right,
                $right.calculate_value(),
                $right.calculate_value(),
                $right.find_starting_edge(),
                $right.find_starting_edge_v3(),
            );
        }};
    }

    #[test]
    fn test_find_starting_edge() {
        assert_find_starting_edge_eq_all_versions!(0, Tile::new([1, 1, 1, 1]));
        assert_find_starting_edge_eq_all_versions!(1, Tile::new([2, 1, 1, 1]));
        assert_find_starting_edge_eq_all_versions!(2, Tile::new([1, 2, 1, 1]));
        assert_find_starting_edge_eq_all_versions!(3, Tile::new([1, 1, 2, 1]));

        assert_find_starting_edge_eq_all_versions!(1, Tile::new([4, 1, 2, 3]));
        assert_find_starting_edge_eq_all_versions!(2, Tile::new([3, 4, 1, 2]));
        assert_find_starting_edge_eq_all_versions!(3, Tile::new([2, 3, 4, 1]));

        assert_find_starting_edge_eq_all_versions!(1, Tile::new([4, 2, 2, 3]));
        assert_find_starting_edge_eq_all_versions!(2, Tile::new([3, 4, 2, 2]));
        assert_find_starting_edge_eq_all_versions!(3, Tile::new([2, 3, 4, 2]));

        assert_find_starting_edge_eq_all_versions!(0, Tile::new([2, 2, 2, 2]));
        assert_find_starting_edge_eq_all_versions!(0, Tile::new([1, 2, 3, 4]));

        assert_find_starting_edge_eq_all_versions!(3, Tile::new([4, 3, 2, 1]));
        assert_find_starting_edge_eq_all_versions!(1, Tile::new([2, 1, 3, 4]));
    }

    #[bench]
    fn bench_calculate_value(b: &mut Bencher) {
        let mut _t = 0;
        b.iter(black_box(|| {
            Tile::all_permutations(1..=4).for_each(|edge| {
                _t = Tile::from_tuple(edge).calculate_value();
            });
        }));
    }

    #[bench]
    fn bench_find_starting_edge(b: &mut Bencher) {
        let mut _t = 0;
        b.iter(black_box(|| {
            Tile::all_permutations(1..=4).for_each(|edge| {
                _t = Tile::from_tuple(edge).find_starting_edge();
            });
        }));
    }

    #[bench]
    fn bench_find_starting_edge_v1(b: &mut Bencher) {
        let mut _t = 0;
        b.iter(black_box(|| {
            Tile::all_permutations(1..=4).for_each(|edge| {
                _t = Tile::from_tuple(edge).find_starting_edge_v1();
            });
        }));
    }

    #[bench]
    fn bench_find_starting_edge_v2(b: &mut Bencher) {
        let mut _t = 0;
        b.iter(black_box(|| {
            Tile::all_permutations(1..=4).for_each(|edge| {
                _t = Tile::from_tuple(edge).find_starting_edge_v2();
            });
        }));
    }

    #[bench]
    fn bench_find_starting_edge_v3(b: &mut Bencher) {
        let mut _t = 0;
        b.iter(black_box(|| {
            Tile::all_permutations(1..=4).for_each(|edge| {
                _t = Tile::from_tuple(edge).find_starting_edge_v3();
            });
        }));
    }
}

use std::hash::{Hash, Hasher};
impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let idx = self.find_starting_edge() as usize;
        (0..4).for_each(|i| self.edges[(idx + i) % 4].hash(state))
    }
}

#[cfg(test)]
mod tests_hash {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    extern crate test;
    use core::hint::black_box;
    use test::Bencher;

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
                "{:?} == {:?} => calculate_value() {:?} == {:?} => find_starting_edge() {:?} == {:?} => calculate_hash() {:?} == {:?}",
                &$left,
                &$right,
                $left.calculate_value(),
                $right.calculate_value(),
                $left.find_starting_edge(),
                $right.find_starting_edge(),
                calculate_hash(&$left),
                calculate_hash(&$right),
            );
        }};
    }

    macro_rules! assert_hash_ne {
        ($left:expr, $right:expr $(,)?) => {{
            assert_ne!(
                calculate_hash(&$left),
                calculate_hash(&$right),
                "{:?} == {:?} => calculate_value() {:?} == {:?} => find_starting_edge() {:?} == {:?} => calculate_hash() {:?} == {:?}",
                &$left,
                &$right,
                $left.calculate_value(),
                $right.calculate_value(),
                $left.find_starting_edge(),
                $right.find_starting_edge(),
                calculate_hash(&$left),
                calculate_hash(&$right),
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

    #[bench]
    fn bench_calculate_hash(b: &mut Bencher) {
        let mut _t = 0;
        b.iter(black_box(|| {
            Tile::all_permutations(1..=4).for_each(|edge| {
                _t = calculate_hash(&Tile::from_tuple(edge));
            });
        }));
    }
}
