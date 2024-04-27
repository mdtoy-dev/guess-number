use rand::Rng;
use std::io;

fn main() {
    let n = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number (1-100):");

    let mut guesses = 0;

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        guesses += 1;

        if guess > n {
            println!("Lower");
        } else if guess < n {
            println!("Higher");
        } else {
            println!("Yes, it was {}. How did you know?", n);
            break;
        }

        if guesses == 3 {
            println!("Out of guesses. The number was {}.", n);
            break;
        }
    }
}
