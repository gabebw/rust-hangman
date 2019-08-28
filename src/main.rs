use std::io::{self, BufRead};
use std::process;

fn ask() -> String {
    println!("Your guess (one letter please)?");
    let stdin = io::stdin();
    let result = stdin.lock().lines().next().unwrap().unwrap().trim_end().to_string();
    result
}

#[derive(Debug, PartialEq)]
enum GuessResult {
    CorrectGuess,
    RepeatedCorrectGuess,
    WrongGuess,
    RepeatedWrongGuess,
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

    fn guess(&mut self, guess: &str) -> GuessResult {
        let matches_secret = self.secret.find(&guess).is_some();
        let already_guessed = self.guessed.contains(&guess.to_string());
        if matches_secret {
            if already_guessed {
                GuessResult::RepeatedCorrectGuess
            } else {
                self.guessed.push(guess.to_string());
                self.num_guesses += 1;
                GuessResult::CorrectGuess
            }
        } else {
            if already_guessed {
                GuessResult::RepeatedWrongGuess
            } else {
                self.guessed.push(guess.to_string());
                self.num_guesses += 1;
                self.num_bad_guesses += 1;
                GuessResult::WrongGuess
            }
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

        match game.guess(&guess) {
            GuessResult::CorrectGuess | GuessResult::RepeatedCorrectGuess => {
                println!("Yep, that's in there");
                if game.is_won() {
                    println!("You win!");
                    process::exit(0);
                }
            },
            GuessResult::WrongGuess | GuessResult::RepeatedWrongGuess => {
                println!("Nope, not in there");
            }
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
        assert_eq!(GuessResult::WrongGuess, result);
        assert_eq!(1, game.num_guesses);
        assert_eq!(1, game.num_bad_guesses);
    }

    #[test]
    fn test_repeated_bad_guess() {
        let mut game = Game::build("hello");
        game.guess("F");
        let result = game.guess("F");
        assert_eq!(GuessResult::RepeatedWrongGuess, result);
        assert_eq!(1, game.num_guesses);
        assert_eq!(1, game.num_bad_guesses);
    }

    #[test]
    fn test_good_guess() {
        let mut game = Game::build("hello");
        let result = game.guess("h");
        assert_eq!(GuessResult::CorrectGuess, result);
        assert_eq!(1, game.num_guesses);
        assert_eq!(0, game.num_bad_guesses);
    }

    #[test]
    fn test_repeated_good_guess() {
        let mut game = Game::build("hello");
        game.guess("h");
        let result = game.guess("h");
        assert_eq!(GuessResult::RepeatedCorrectGuess, result);
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
