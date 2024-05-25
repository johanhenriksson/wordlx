use crate::dictionary::Dictionary;
use crate::word::Word;
use lazy_static::lazy_static;

lazy_static! {
    static ref WORDS: Dictionary = Dictionary::game_words();
    static ref GUESSES: Dictionary = Dictionary::valid_guesses();
}

#[derive(PartialEq, Debug, Default)]
pub enum Phase {
    #[default]
    Playing,
    Won,
    Lost,
}

#[derive(PartialEq, Debug)]
pub enum Input {
    Character(char),
    Enter,
    Backspace,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Error {
    #[default]
    None,
    InvalidGuess,
}

#[derive(Debug, Default)]
pub struct GameState {
    pub phase: Phase,
    pub answer: Word,
    pub guess: Word,
    pub guesses: Vec<Word>,
    pub error: Error,
}

impl GameState {
    pub fn new(answer: &str) -> Self {
        Self {
            answer: Word::new(answer),
            ..Default::default()
        }
    }

    pub fn new_random() -> Self {
        let answer = WORDS.random();
        println!("New game: {}", answer);
        Self::new(&answer.to_string())
    }

    pub fn input(&mut self, input: Input) {
        if self.phase != Phase::Playing {
            return;
        }
        match input {
            Input::Character(c) => {
                self.guess.put(c);
                self.error = Error::None;
            }
            Input::Backspace => {
                self.guess.erase();
                self.error = Error::None;
            }
            Input::Enter => self.submit(),
        }
    }

    pub fn full(&self) -> bool {
        self.guesses.len() == 6
    }

    fn submit(&mut self) {
        if !self.guess.valid() {
            return;
        }
        if self.full() {
            return;
        }

        let guess = self.guess.clone();
        if !(WORDS.contains(&guess.to_string()) || GUESSES.contains(&guess.to_string())) {
            self.error = Error::InvalidGuess;
            return;
        }
        self.error = Error::None;

        self.guess = Word::empty();
        self.guesses.push(guess.clone());

        if self.full() {
            self.phase = Phase::Lost;
        }
        if guess == self.answer {
            self.phase = Phase::Won;
        }
    }
}
