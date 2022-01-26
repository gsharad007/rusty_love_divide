#![feature(test)]
#![feature(bench_black_box)]

mod acceptance_tests;
mod game;

use game::deck::Deck;
use game::deck_generator::DeckGenerator;
// use game::tile::Tile;

fn main() {
    let game_deck = Deck::generate_all_permutations_and_root();
    println!("Hello, world! {0}", game_deck.len());
}

#[cfg(test)]
mod tests_main {
    use super::*;
    extern crate test;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_verify_testing() {
        assert_eq!(2 + 2, 4);
    }
}
