use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Rust learning: Guess my number game!");
    println!("I'm thinking of a number between 1 and 25. Try to guess it!");

    let max_attempts: u32 = 5;
    println!("You have {} attempts to guess the number.", max_attempts);

    let secret_number: u32 = rand::rng().random_range(..=25);
    let mut attempts: u32 = max_attempts;

    while attempts > 0 {
        println!("Attempts left: {}", attempts);
        println!("Guess a number between 1 and 25:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // if guess == secret_number {
        //     println!(
        //         "Congratulations! You guessed the magic number {} correctly!",
        //         guess
        //     );
        //     break;
        // } else if guess < secret_number {
        //     println!("Too low. Try again!");
        // } else {
        //     println!("Too high. Try again!");
        // }

        // Ordering
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!(
                    "Congratulations! You guessed the magic number {} correctly!",
                    secret_number
                );
                return;
            }
        }

        attempts -= 1;

        if attempts == 0 {
            println!(
                "You've run out of attempts. The secret number was {}. Better luck next time!",
                secret_number
            );
        }
    }
}
