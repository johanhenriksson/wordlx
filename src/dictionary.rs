use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashSet;

use crate::word::Word;

lazy_static! {
    pub static ref WORDS: Dictionary = Dictionary::game_words();
    pub static ref GUESSES: Dictionary = Dictionary::valid_guesses();
}

pub struct Dictionary(HashSet<Word>);

impl Dictionary {
    fn game_words() -> Self {
        let words = include_str!("../wordle-valid.txt")
            .lines()
            .map(|s| Word::new(s))
            .collect();
        Self(words)
    }

    fn valid_guesses() -> Self {
        let words = include_str!("../wordle-guess.txt")
            .lines()
            .map(|s| Word::new(s))
            .collect();
        Self(words)
    }

    pub fn contains(&self, word: Word) -> bool {
        self.0.contains(&word)
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<Word> {
        self.0.iter()
    }

    pub fn random(&self) -> Word {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..self.0.len());
        let index = rng.gen_range(0..self.0.len());
        self.0.iter().nth(index).unwrap().clone()
    }
}

impl IntoIterator for Dictionary {
    type Item = Word;
    type IntoIter = std::collections::hash_set::IntoIter<Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
