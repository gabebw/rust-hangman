#[derive(Debug, PartialEq)]
pub enum GuessResult {
    CorrectGuess,
    WrongGuess,
}

pub struct Game<'a> {
    secret: &'a str,
    guessed: Vec<String>,
    pub num_bad_guesses: usize,
    pub num_guesses: usize,
}

impl<'a> Game<'a> {
    pub fn build(secret: &str) -> Game {
        Game {
            secret,
            guessed: Vec::new(),
            num_bad_guesses: 0,
            num_guesses: 0
        }
    }

    pub fn display_status(&self) -> () {
        println!("Total guesses: {} / Bad guesses: {}", self.num_guesses, self.num_bad_guesses);
        println!("Number of bad guesses: {}", self.num_bad_guesses);
    }

    pub fn display_phrase(&self) -> () {
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

    pub fn guess(&mut self, guess: &str) -> GuessResult {
        let matches_secret = self.secret.find(&guess).is_some();
        let already_guessed = self.guessed.contains(&guess.to_string());
        if matches_secret {
            if already_guessed {
                GuessResult::CorrectGuess
            } else {
                self.guessed.push(guess.to_string());
                self.num_guesses += 1;
                GuessResult::CorrectGuess
            }
        } else {
            if already_guessed {
                GuessResult::WrongGuess
            } else {
                self.guessed.push(guess.to_string());
                self.num_guesses += 1;
                self.num_bad_guesses += 1;
                GuessResult::WrongGuess
            }
        }
    }

    pub fn is_won(&self) -> bool {
        self.secret.chars().all(|c|
            c.is_whitespace() || self.guessed.contains(&c.to_string())
        )
    }
}
