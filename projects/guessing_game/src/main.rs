use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess a number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The Number was {secret_number}");

    loop{
        println!("Please Input a number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Read Line");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You Guessed {guess}");
        

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
