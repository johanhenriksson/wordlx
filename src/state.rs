use crate::word::Word;

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

#[derive(Debug, Default)]
pub struct GameState {
    pub phase: Phase,
    pub answer: Word,
    pub guess: Word,
    pub guesses: Vec<Word>,
}

impl GameState {
    pub fn new(answer: &str) -> Self {
        Self {
            answer: Word::new(answer),
            ..Default::default()
        }
    }

    pub fn input(&mut self, input: Input) {
        if self.phase != Phase::Playing {
            return;
        }
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
        if !self.guess.valid() {
            return;
        }
        if self.full() {
            return;
        }

        let guess = self.guess.clone();
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
