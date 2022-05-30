use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("Secret number: {secret_number}");

    loop {
        println!("Guess a number:");

        // mutable variable
        let mut guess = String::new();

        // read input, handle error case (expect crashes on error)
        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number.");
                continue;
            }
        };

        // println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("low guess"),
            Ordering::Greater => println!("high guess"),
            Ordering::Equal => {
                println!("correct!");
                break;
            }
        }
    }
}
