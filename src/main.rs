mod game;
mod acceptance_tests;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_verify_testing() {
    assert_eq!(2 + 2, 4);
}
