extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

pub struct Hangman {
    secret_words: [String; 3],
    current_secret_word: String,
    number_of_guesses: i32,
    remaining_attempts: usize,
    guessed_chars: Vec<char>,
}

impl Hangman {
    pub fn new() -> Self {
        Hangman {
            secret_words: [
                "alex".to_string(),
                "programmer".to_string(),
                "test".to_string(),
            ],
            current_secret_word: String::new(),
            number_of_guesses: 0,
            remaining_attempts: 0,
            guessed_chars: Vec::new(),
        }
    }

    pub fn begin_game(&mut self) -> () {
        let mut rng = thread_rng();
        let choice = self.secret_words.choose(&mut rng).unwrap();
        self.current_secret_word = choice.to_string();
        self.remaining_attempts = choice.len();
        println!("CURRENT SECRET WORD: {}", self.current_secret_word);
        self.show_word_blanks();
        println!("\n");
        self.show_remaining_guesses();
        self.prompt_for_input()
    }

    fn show_word_blanks(&self) -> () {
        let mut formatted_blanks = String::new();
        let chars = self.current_secret_word.chars();

        println!("\nHere is the word:\n");
        for _ in chars {
            formatted_blanks.push_str("____  ");
        }

        println!("{}", formatted_blanks)
    }

    fn show_remaining_guesses(&self) -> () {
        println!("Remaining guesses: {}", self.remaining_attempts)
    }

    fn prompt_for_input(&mut self) -> () {
        println!("Enter a letter:");

        let mut guessed = String::new();
        io::stdin()
            .read_line(&mut guessed)
            .expect("Failed to read input");

        let guessed_char = guessed.chars().next().unwrap();

        let found = self
            .current_secret_word
            .chars()
            .find(|&c| c == guessed_char);

        match found {
            Some(c) => self.guessed_chars.push(c),
            None => {
                self.number_of_guesses += 1;
                self.remaining_attempts -= 1;
            }
        }
    }
}
