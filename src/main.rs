use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        print!("guess a number: ");
        io::stdout().flush().unwrap(); // make sure previous line is written to the screen

        let mut guess = String::new(); // mutable variable

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number.");
                continue;
            }
        };

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
