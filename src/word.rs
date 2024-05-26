use std::{fmt::Display, hash::Hash, hash::Hasher};

use serde::{Deserialize, Serialize};

use crate::charset::Charset;

const MASK: u32 = 0b11111;

fn char_bits(c: char) -> u32 {
    if c < 'a' || c > 'z' {
        return 0;
    }
    return c as u32 - b'a' as u32 + 1;
}

fn char_from_bits(bits: u32) -> char {
    if bits == 0 {
        return ' ';
    }
    let bits = bits - 1;
    (b'a' + bits as u8) as char
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Word(u32);

impl Word {
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn new(s: &str) -> Self {
        let mut word = Self::empty();
        for (i, c) in s.chars().take(5).enumerate() {
            word.set(i, c);
        }
        word
    }

    pub fn set(&mut self, i: usize, c: char) {
        if i >= 5 {
            return;
        }
        let offset = i * 5;
        let mask = !(MASK << offset);
        self.0 &= mask; // remove any existing bits
        self.0 |= char_bits(c) << offset;
    }

    pub fn contains(&self, c: char) -> bool {
        let cbits = char_bits(c);
        let mut wordbits = self.0;
        for _ in 0..5 {
            if wordbits & MASK == cbits {
                return true;
            }
            wordbits = wordbits >> 5;
        }
        false
    }

    pub fn at(&self, i: usize) -> char {
        let offset = i * 5;
        let bits = (self.0 >> offset) & MASK;
        return char_from_bits(bits);
    }

    pub fn charset(&self) -> Charset {
        let mut set = Charset::none();
        for c in self {
            set.include(c);
        }
        set
    }

    pub fn bits(&self) -> u32 {
        self.0
    }

    pub fn iter(&self) -> WordIter {
        WordIter::new(*self)
    }
}

impl Default for Word {
    fn default() -> Self {
        Self::empty()
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Word {}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iter().collect::<String>())
    }
}

impl std::fmt::Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Hash for Word {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

pub struct WordIter {
    word: Word,
    mask: u32,
    shift: u32,
}

impl WordIter {
    pub fn new(word: Word) -> Self {
        Self {
            word,
            mask: MASK,
            shift: 0,
        }
    }
}

impl Iterator for WordIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.mask > MASK << 20 {
            return None;
        }
        let bits = (self.word.0 & self.mask) >> self.shift;
        self.mask <<= 5;
        self.shift += 5;
        Some(char_from_bits(bits))
    }

    fn count(self) -> usize {
        5
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= 5 {
            return None;
        }
        return Some(self.word.at(n));
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (5, Some(5))
    }
}

impl IntoIterator for &Word {
    type Item = char;
    type IntoIter = WordIter;

    fn into_iter(self) -> Self::IntoIter {
        return self.iter();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_word_new() {
        let word = Word::new("hello");
        assert_eq!(word.at(0), 'h');
        assert_eq!(word.at(1), 'e');
        assert_eq!(word.at(2), 'l');
        assert_eq!(word.at(3), 'l');
        assert_eq!(word.at(4), 'o');
    }

    #[test]
    fn test_word_eq() {
        let word = Word::new("hello");
        assert_eq!(word, Word::new("hello"));
        assert_ne!(word, Word::new("h llo"));
        assert_ne!(word, Word::new("he lo"));
        assert_ne!(word, Word::new("hel o"));
        assert_ne!(word, Word::new("hell "));

        assert_ne!(Word::empty(), Word::new("aaaaa"));
    }

    #[test]
    fn test_word_set() {
        let mut word = Word::empty();
        word.set(0, 'h');
        word.set(1, 'e');
        word.set(2, 'l');
        word.set(3, 'l');
        word.set(4, 'o');
        assert_eq!(word, Word::new("hello"));
    }

    #[test]
    fn test_word_iter() {
        let word = Word::new("hello");
        let mut iter = word.iter();
        assert_eq!(iter.next(), Some('h'));
        assert_eq!(iter.next(), Some('e'));
        assert_eq!(iter.next(), Some('l'));
        assert_eq!(iter.next(), Some('l'));
        assert_eq!(iter.next(), Some('o'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_word_contains() {
        let word = Word::new("hello");
        assert_eq!(word.contains('h'), true);
        assert_eq!(word.contains('e'), true);
        assert_eq!(word.contains('l'), true);
        assert_eq!(word.contains('o'), true);
        assert_eq!(word.contains('x'), false);
        assert_eq!(word.contains('j'), false);
        assert_eq!(word.contains('d'), false);
    }

    #[test]
    fn test_word_to_charset() {
        let word = Word::new("hello");
        let set = word.charset();
        assert_eq!(set, Charset::from_str("hello"));
    }
}
