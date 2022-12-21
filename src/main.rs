// This is a guessing game program.
// 2022-12-21
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("-- GUESS THE NUMBER --");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line...");

        let guess: u32 = guess.trim().parse().expect("Plase type a number!");

        println!("You guessed: {}", guess);

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
