use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the numberrr!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_err) => {
                println!("{}, ----- Not a valid input", _err);
                continue;
            }
        };

        println!("You guessed, {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}, Too small!", guess),
            Ordering::Greater => println!("{}, Too big!", guess),
            Ordering::Equal => {
                println!("{} is correct! You win!", guess);
                break;
            }
        }

        println!("Please input your guess.");
    }
}
