// This is a guessing game program.
// 2022-12-21
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("-- GUESS THE NUMBER --"); // welcome the user to the game

    // creates a random number that the user has to guess, from 1 to 100.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // starts an infinite loop. Will be interrupted with "break"
    loop {
        println!("Please input your guess:");

        // declares a mutable empty string called guess
        let mut guess = String::new();

        // creates a handler to standart input.
        // uses a mutable reference to the guess variable.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line...");

        // shadows the previous declared "guess" variable.
        // transforms the string into a positive integer of 32 bits.
        // we use match, if the cast is successfull returns the var "num", else it returns the error message.
        // in case of error, the "continue" keyword interrupts the current iteration and the loop starts again.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // compares the secret number to the guess.
        // if it is equal, shows a message to the user and the "break" keyword breaks the current iteration and exits the loop.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!!!"),
            Ordering::Greater => println!("Too hight!!!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
        }
    }
}
