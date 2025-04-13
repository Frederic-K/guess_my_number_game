use rand::Rng;
use std::io;

fn main() {
    println!("Rust learning: Guess my number game!");
    println!("I'm thinking of a number between 1 and 25. Try to guess it!");

    let max_attempts: u32 = 5;
    println!("You have {} attempts to guess the number.", max_attempts);

    let secret_number: u32 = rand::rng().random_range(..=25);
    let mut attempts_taken: u32 = 0;
}
