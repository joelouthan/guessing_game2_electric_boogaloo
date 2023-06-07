// Purpose: guessing number game from 1 to 100
// Author: Joseph Louthan
// Date: 08/31/2021
// License: MIT
// Notes: This is a practice project from the Rust book
//       https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
//       I am using this to learn Rust
//       I am also using this to learn git and github
//       I am also using this to learn Neovim

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // generate random number
    let secret_number = rand::thread_rng().gen_range(1..101);

    // loop until user guesses correctly
    loop {
        // get user input
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // convert user input to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // print user input
        println!("You guessed: {}", guess);

        // compare user input to random number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // user guessed correctly
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
