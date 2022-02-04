use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read stdin");

        let guess: u32 = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To low!"),
            Ordering::Equal => {
                println!("You got it!!!");
                break;
            }
            Ordering::Greater => println!("To high!"),
        }
    }
}
