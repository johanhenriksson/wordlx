use std::{fmt::Display, hash::Hash, hash::Hasher};

use crate::guess::Charset;

#[derive(Copy, Clone, Debug)]
pub struct Word {
    chars: [char; 5],
    index: usize,
}

impl Word {
    pub fn empty() -> Self {
        Self {
            chars: [' '; 5],
            index: 0,
        }
    }

    pub fn invalid() -> Self {
        Self {
            chars: ['-'; 5],
            index: 0,
        }
    }

    pub fn new(s: &str) -> Self {
        let mut chars = [' '; 5];
        for (i, c) in s.chars().enumerate() {
            if i > 5 {
                break;
            }
            chars[i] = c;
        }
        Self {
            chars,
            index: s.len(),
        }
    }

    pub fn valid(&self) -> bool {
        self.index == 5
    }

    pub fn put(&mut self, c: char) {
        if self.index == 5 {
            return;
        }
        self.chars[self.index] = c;
        self.index += 1;
    }

    pub fn erase(&mut self) {
        if self.index == 0 {
            return;
        }
        self.index -= 1;
        self.chars[self.index] = ' ';
    }

    pub fn contains(&self, c: char) -> bool {
        self.chars.contains(&c)
    }

    pub fn at(&self, i: usize) -> char {
        self.chars[i]
    }

    pub fn charset(&self) -> Charset {
        let mut set = Charset::none();
        for c in &self.chars {
            set.include(*c);
        }
        set
    }
}

impl Default for Word {
    fn default() -> Self {
        Self::empty()
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.chars == other.chars
    }
}

impl Eq for Word {}

impl IntoIterator for Word {
    type Item = char;
    type IntoIter = std::array::IntoIter<char, 5>;

    fn into_iter(self) -> Self::IntoIter {
        self.chars.into_iter()
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.chars {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl Hash for Word {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.chars.hash(state);
    }
}
