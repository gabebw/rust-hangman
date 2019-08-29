use std::io::{self, BufRead};
use std::process;
mod game;

fn ask() -> String {
    println!("Your guess (one letter please)?");
    let stdin = io::stdin();
    let result = stdin.lock().lines().next().unwrap().unwrap().trim_end().to_string();
    result
}

fn main() {
    let thumbs_up = "\u{1F44D}";
    let red_x = "\u{274C}";
    let secret = "HELLO WORLD";
    let mut game = game::Game::build(secret);
    // head, 2 arms, 2 legs, body = 6 bad guesses allowed, and 7th ends the game
    while game.num_bad_guesses < 7 {
        game.display_status();
        game.display_phrase();
        let guess = ask();

        match game.guess(&guess) {
            game::GuessResult::CorrectGuess => {
                println!("{} Correct guess", thumbs_up);
                if game.is_won() {
                    println!("You win!");
                    process::exit(0);
                }
            },
            game::GuessResult::WrongGuess => {
                // There's an extra space after the emoji because my terminal (Alacritty) doesn't
                // display any space between the emoji and text if there's only one.
                println!("{}  Nope, try again", red_x);
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
        let mut game = game::Game::build("hello");
        let result = game.guess("F");
        assert_eq!(game::GuessResult::WrongGuess, result);
        assert_eq!(1, game.num_guesses);
        assert_eq!(1, game.num_bad_guesses);
    }

    #[test]
    fn test_repeated_bad_guess() {
        let mut game = game::Game::build("hello");
        game.guess("F");
        let result = game.guess("F");
        assert_eq!(game::GuessResult::WrongGuess, result);
        assert_eq!(1, game.num_guesses);
        assert_eq!(1, game.num_bad_guesses);
    }

    #[test]
    fn test_good_guess() {
        let mut game = game::Game::build("hello");
        let result = game.guess("h");
        assert_eq!(game::GuessResult::CorrectGuess, result);
        assert_eq!(1, game.num_guesses);
        assert_eq!(0, game.num_bad_guesses);
    }

    #[test]
    fn test_repeated_good_guess() {
        let mut game = game::Game::build("hello");
        game.guess("h");
        let result = game.guess("h");
        assert_eq!(game::GuessResult::CorrectGuess, result);
        assert_eq!(1, game.num_guesses);
        assert_eq!(0, game.num_bad_guesses);
    }

    #[test]
    fn test_winning() {
        let mut game = game::Game::build("hi");
        game.guess("h");
        game.guess("i");
        assert_eq!(true, game.is_won());
    }

    #[test]
    fn test_winning_with_whitespace() {
        let mut game = game::Game::build("h i");
        game.guess("h");
        game.guess("i");
        assert_eq!(true, game.is_won());
    }
}
