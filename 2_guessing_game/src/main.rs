use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Enter your number");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    println!("Input your guess: ");

    loop {
        /* 
         * Variables are immutable by default e.g. let apples = 5;  
         * Added mut for mutable e.g. let mut apple = 5;
         * String::new() = new String() in Java
        */
        let mut guess = String::new();

        // Could use std::io::stdin if no use statement at the start
        io::stdin()
            .read_line(&mut guess) // & indicates reference object -> let multiple parts of code access
                                   // this data w/o copying data into memory multiple times
                                   // references are mutable too...that's why we write &mut
            .expect("Failed to read line!");

        /*
         * Parse returns an enum => variants of Ok and Err
         * match for to match each variant to an action like in Ordering
         * Err(_) where _ means catch all errors
         */
        let guess: u32 = match guess.trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
