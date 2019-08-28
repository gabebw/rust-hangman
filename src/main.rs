use std::io::{self, BufRead};
use std::process;

fn ask() -> String {
    let stdin = io::stdin();
    let result = stdin.lock().lines().next().unwrap().unwrap().trim_end().to_string();
    result
}

fn main() {
    let secret = "HELLO WORLD";

    let mut guessed: Vec<String> = Vec::new();
    let mut num_bad_guesses = 0;
    let mut num_guesses = 0;
    // head, 2 arms, 2 legs, body = 6 bad guesses allowed, and 7th ends the game
    while num_bad_guesses < 7 {
        print!("Phrase: ");
        for c in secret.chars() {
            if guessed.contains(&c.to_string()) || c.is_whitespace() {
                print!("{}", c);
            } else {
                print!("_");
            }
        }
        println!("");
        println!("Total guesses: {} / Bad guesses: {}", num_guesses, num_bad_guesses);
        println!("Your guess (one letter please)?");
        println!("Number of bad guesses: {}", num_bad_guesses);
        let guess = ask();
        if let Some(_) = secret.find(&guess) {
            println!("Yep, that's in there");
            guessed.push(guess.to_string());

            if secret.chars().all(|c| c.is_whitespace() || guessed.contains(&c.to_string())) {
                println!("You win!");
                process::exit(0);
            }
        } else {
            println!("Nope, not in there");
            num_bad_guesses += 1;
        }
        num_guesses += 1;
    }
    println!("You lose :(");
}
