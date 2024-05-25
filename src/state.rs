use crate::dictionary::{GUESSES, WORDS};
use crate::word::Word;

#[derive(PartialEq, Debug)]
pub enum Phase {
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

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    None,
    InvalidGuess,
}

#[derive(Debug)]
pub struct GameState {
    pub phase: Phase,
    pub answer: Word,
    pub guess: Word,
    pub guesses: Vec<Word>,
    pub error: Error,
}

impl GameState {
    pub fn new(answer: &str) -> Self {
        if answer.len() != 5 {
            panic!("Answer must be 5 characters long");
        }

        let answer = Word::new(answer);
        Self {
            answer,
            phase: Phase::Playing,
            error: Error::None,
            guess: Word::empty(),
            guesses: Vec::new(),
        }
    }

    pub fn new_random() -> Self {
        let answer = WORDS.random();
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
