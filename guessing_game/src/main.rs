use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    
    loop {
        println!("Please input you guess:");

        let mut guess = String::new();
        // in rust variables are immutable by default
        // to make them mutable we need to use the mut keyword

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // trim removes the whitespace and parse converts the string to a number

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!!".red()),
            Ordering::Greater => println!("{}", "Too big!!".red()),
            Ordering::Equal => {
                println!("{}", "You win!!".green());
                break;
            },
            
        }
    }
}
