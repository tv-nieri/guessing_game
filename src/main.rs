// This is a guessing game program.
// 2022-12-21
use std::io;

fn main() {
    println!("-- GUESS THE NUMBER --");

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line...");

    println!("You guessed: {}", guess);
}
