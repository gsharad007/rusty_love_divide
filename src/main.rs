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

#[test]
fn test_verify_testing() {
    assert_eq!(2 + 2, 4);
}
