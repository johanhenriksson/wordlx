use crate::charset::Charset;
use crate::word::Word;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WordSpace([Charset; 5]);

impl WordSpace {
    pub fn new() -> Self {
        WordSpace([Charset::all(); 5])
    }

    pub fn exclude(&mut self, i: usize, c: char) {
        self.0[i].exclude(c);
    }

    pub fn only(&mut self, i: usize, c: char) {
        self.0[i] = Charset::none().include(c);
    }

    pub fn matches(&self, word: &Word) -> bool {
        word.into_iter()
            .enumerate()
            .all(|(i, c)| self.0[i].includes(c))
    }
}

#[derive(Debug, Clone)]
pub struct WordFilter {
    answer: Word,

    pub accepted: Charset,
    pub required: Charset,
    pub space: WordSpace,
    pub correct: Word,
}

impl WordFilter {
    pub fn new(answer: Word) -> Self {
        Self {
            answer: answer.clone(),
            accepted: Charset::all(),
            required: Charset::none(),
            space: WordSpace::new(),
            correct: Word::empty(),
        }
    }

    pub fn apply(&mut self, guess: Word) {
        // update mask
        for (i, c) in guess.into_iter().enumerate() {
            if c == self.answer.at(i) {
                // correct character in correct position
                self.correct.set(i, c);
                self.required.include(c);
                self.space.only(i, c);
            } else if self.answer.contains(c) {
                // correct character in wrong position
                self.required.include(c);
                self.space.exclude(i, c);
            } else {
                // incorrect character
                self.accepted.exclude(c);
                for i in 0..5 {
                    self.space.exclude(i, c);
                }
            }
        }
    }

    pub fn matches(&self, word: &Word) -> bool {
        let wm = word.charset();

        // ensure we dont have any rejected characters
        if !self.accepted.contains(wm) {
            return false;
        }

        // ensure we have all required characters
        if !wm.contains(self.required) {
            return false;
        }

        // ensure the word has no characters in known wrong positions
        self.space.matches(word)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wordspace() {
        let mut space = WordSpace::new();
        assert_eq!(space.matches(&Word::new("abcde")), true);
        assert_eq!(space.matches(&Word::new("bcdea")), true);
        space.exclude(0, 'a');
        assert_eq!(space.matches(&Word::new("abcde")), false);
    }

    #[test]
    fn test_wordspace_only() {
        let mut space = WordSpace::new();
        space.only(0, 'a');
        assert_eq!(space.0[0].includes('a'), true);
        assert_eq!(space.0[0].includes('b'), false);
        assert_eq!(space.matches(&Word::new("abcde")), true);
        assert_eq!(space.matches(&Word::new("bbcde")), false);
    }

    #[test]
    fn test_filter() {
        let mut filter = WordFilter::new(Word::new("theta"));
        filter.apply(Word::new("beast"));
        filter.apply(Word::new("tears"));
        filter.apply(Word::new("tamed"));
        assert_eq!(filter.matches(&Word::new("theta")), true);
        assert_eq!(filter.matches(&Word::new("steal")), false);
        assert_eq!(filter.matches(&Word::new("steak")), false);
    }
}
