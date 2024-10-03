use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io::stdin};

fn main() {
    loop {
        let secret_number = rand::thread_rng().gen_range(1, 50);
        println!("{}", "Guess the number game!".yellow());

        loop {
            println!("{}", "Enter your guess(1-50)".blue());

            let mut guess = String::new();
            stdin().read_line(&mut guess).expect("Something is wrong!");

            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}", "Please enter a valid number!".red());
                    continue;
                }
            };

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("{}", "Too Small!".red()),
                Ordering::Greater => println!("{}", "Too big!".red()),
                Ordering::Equal => {
                    println!("{}", "You win!".green());
                    break;
                }
            }
        }

        // Ask the user if they want to play again
        println!("{}", "Do you want to play again? (yes/no)".blue());

        let mut play_again = String::new();
        stdin()
            .read_line(&mut play_again)
            .expect("Something is wrong!");

        if play_again.trim().to_lowercase() != "yes" {
            println!("{}", "Thanks for playing!".green());
            break; // Exit the main loop and end the game
        }
    }
}
