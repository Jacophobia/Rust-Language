use std::io::{self, Write};
use rand::Rng;

fn main() {
    println!("Welcome to the number guessing game.");
    println!("To win you must guess the randomly generated number.");
    print!("Guess a number between 1 and 100: ");
    io::stdout().flush().unwrap();

    let rng_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed {}!!", guess.trim());
    println!("The number was {}!", rng_number)
}
