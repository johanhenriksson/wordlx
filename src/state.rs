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
    pub guess: Guess,
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
            guesses: Vec::new(),
            guess: Guess::new(),
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
        self.error = Error::None;
        match input {
            Input::Character(c) => self.guess.put(c),
            Input::Backspace => self.guess.erase(),
            Input::Enter => self.submit(),
        }
    }

    pub fn full(&self) -> bool {
        self.guesses.len() == 6
    }

    fn submit(&mut self) {
        if self.phase != Phase::Playing {
            return;
        }
        if !self.guess.complete() {
            return;
        }
        if !self.guess.valid() {
            self.error = Error::InvalidGuess;
            return;
        }

        let guess: Word = self.guess.clone().into();
        self.guess.clear();
        self.guesses.push(guess.clone());

        if guess == self.answer.into() {
            self.phase = Phase::Won;
        } else if self.full() {
            self.phase = Phase::Lost;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Guess {
    chars: Vec<char>,
}

impl Guess {
    pub fn new() -> Self {
        Self { chars: vec![] }
    }

    pub fn clear(&mut self) {
        self.chars.clear();
    }

    pub fn complete(&self) -> bool {
        self.chars.len() == 5
    }

    // Returns true if the guess is included in the dictionary of valid guesses
    pub fn valid(&self) -> bool {
        let word: Word = self.clone().into();
        WORDS.contains(word) || GUESSES.contains(word)
    }

    pub fn put(&mut self, c: char) {
        if self.chars.len() < 5 {
            self.chars.push(c);
        }
    }

    pub fn erase(&mut self) {
        self.chars.pop();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, char> {
        self.chars.iter()
    }
}

impl From<Guess> for Word {
    fn from(g: Guess) -> Self {
        let mut word = Word::empty();
        for (i, c) in g.chars.iter().enumerate() {
            word.set(i, *c);
        }
        word
    }
}
