use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Welcome to the number guessing game.");
    println!("To win you must guess the randomly generated number.");
    let rng_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        print!("Guess a number between 1 and 100: ");
        io::stdout().flush().expect("Failed to flush stdout.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}!!", guess);

        match guess.cmp(&rng_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("The number was {}!", rng_number);
}
