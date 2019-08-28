use std::io::{self, BufRead};
use std::process;

fn ask() -> String {
    println!("Your guess (one letter please)?");
    let stdin = io::stdin();
    let result = stdin.lock().lines().next().unwrap().unwrap().trim_end().to_string();
    result
}

struct Game<'a> {
    secret: &'a str,
    guessed: Vec<String>,
    num_bad_guesses: usize,
    num_guesses: usize,
}

impl<'a> Game<'a> {
    fn build(secret: &str) -> Game {
        Game {
            secret,
            guessed: Vec::new(),
            num_bad_guesses: 0,
            num_guesses: 0
        }
    }

    fn display_status(&self) -> () {
        println!("Total guesses: {} / Bad guesses: {}", self.num_guesses, self.num_bad_guesses);
        println!("Number of bad guesses: {}", self.num_bad_guesses);
    }

    fn display_phrase(&self) -> () {
        print!("Phrase: ");
        for c in self.secret.chars() {
            if self.guessed.contains(&c.to_string()) || c.is_whitespace() {
                print!("{}", c);
            } else {
                print!("_");
            }
        }
        println!("");
    }

    fn guess(&mut self, guess: &str) -> bool {
        self.guessed.push(guess.to_string());
        self.num_guesses += 1;
        if let Some(_) = self.secret.find(&guess) {
            true
        } else {
            self.num_bad_guesses += 1;
            false
        }
    }

    fn is_won(&self) -> bool {
        self.secret.chars().all(|c|
            c.is_whitespace() || self.guessed.contains(&c.to_string())
        )
    }
}

fn main() {
    let secret = "HELLO WORLD";
    let mut game = Game::build(secret);
    // head, 2 arms, 2 legs, body = 6 bad guesses allowed, and 7th ends the game
    while game.num_bad_guesses < 7 {
        game.display_status();
        game.display_phrase();
        let guess = ask();

        if game.guess(&guess) {
            println!("Yep, that's in there");
            if game.is_won() {
                println!("You win!");
                process::exit(0);
            }
        } else {
            println!("Nope, not in there");
        }
    }
    println!("You lose :(");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bad_guess() {
        let mut game = Game::build("hello");
        let result = game.guess("F");
        assert_eq!(false, result);
        assert_eq!(1, game.num_guesses);
        assert_eq!(1, game.num_bad_guesses);
    }

    #[test]
    fn test_good_guess() {
        let mut game = Game::build("hello");
        let result = game.guess("h");
        assert_eq!(true, result);
        assert_eq!(1, game.num_guesses);
        assert_eq!(0, game.num_bad_guesses);
    }

    #[test]
    fn test_winning() {
        let mut game = Game::build("hi");
        game.guess("h");
        game.guess("i");
        assert_eq!(true, game.is_won());
    }

    #[test]
    fn test_winning_with_whitespace() {
        let mut game = Game::build("h i");
        game.guess("h");
        game.guess("i");
        assert_eq!(true, game.is_won());
    }
}
