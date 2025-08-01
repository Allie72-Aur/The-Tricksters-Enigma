// #![feature(string_remove_matches)]
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!(
        "====================================================\n\
        I'm thinking of a number between 1 and 100 inclusive,\
        \n\t\tcan you guess what it is?"
    );
    // rand::rng() creates a new random number generator
    // random_range(1..=100) generates a random number in the range 1..=100
    let secret_number = rand::rng().random_range(1..=100);
    #[cfg(debug_assertions)]
    println!("The secret number is {secret_number}.");

    loop {
        print!("Enter your guess: ");
        // flush stdout to ensure prompt appears before input
        // this is necessary because stdout is line-buffered
        let _ = io::stdout().flush();
        // guess is a mutable String to hold user input
        // read_line will append the input to the String
        // guess is later reused (i.e. shadowed) to hold the parsed number
        // this is a common pattern in Rust to avoid unnecessary allocations
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Reading a line should not Fail");
        // trim trailing newline and whitespace, convert to number
        // make sure the user cooperates
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please make a valid guess");
                continue;
            }
        };
        #[cfg(debug_assertions)]
        println!("Your guess was {guess}.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("TOO BIG!"),
            Ordering::Equal => {
                println!("You Got IT!! You Win! Congrats 🎉");
                break;
            }
        }
    }
}
