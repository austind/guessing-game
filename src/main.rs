use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    println!("Please input your guess:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input.");
    
    println!("You guessed: {guess}");
    println!("Secret: {secret_number}");
}