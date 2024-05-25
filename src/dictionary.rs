use rand::Rng;
use std::collections::HashSet;

use crate::word::Word;

pub struct Dictionary(HashSet<Word>);

impl Dictionary {
    pub fn game_words() -> Self {
        let words = include_str!("../wordle-valid.txt")
            .lines()
            .map(|s| Word::new(s))
            .collect();
        Self(words)
    }

    pub fn valid_guesses() -> Self {
        let words = include_str!("../wordle-guess.txt")
            .lines()
            .map(|s| Word::new(s))
            .collect();
        Self(words)
    }

    pub fn contains(&self, word: &str) -> bool {
        self.0.contains(&Word::new(word))
    }

    pub fn random(&self) -> Word {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..self.0.len());
        let index = rng.gen_range(0..self.0.len());
        self.0.iter().nth(index).unwrap().clone()
    }
}
