mod hangman;

fn main() {
    let mut hangman = hangman::Hangman::new();
    hangman.begin_game();
}
