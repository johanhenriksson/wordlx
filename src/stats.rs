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

    pub fn matches(&self, word: Word) -> bool {
        word.iter().enumerate().all(|(i, c)| self.0[i].includes(c))
    }
}

#[derive(Debug, Clone)]
pub struct WordFilter {
    answer: Word,

    pub rejected: Charset,
    pub required: Charset,
    pub space: WordSpace,
    pub correct: Word,
}

impl WordFilter {
    pub fn new(answer: Word) -> Self {
        Self {
            answer: answer.clone(),
            rejected: Charset::none(),
            required: Charset::none(),
            space: WordSpace::new(),
            correct: Word::empty(),
        }
    }

    pub fn apply(&mut self, guess: Word) {
        // update mask
        for (i, c) in guess.iter().enumerate() {
            if c == self.answer.at(i) {
                // correct character in correct position
                println!("position {} is {}", i, c);
                self.correct.set(i, c);
                self.required.include(c);
                self.space.only(i, c);
            } else if self.answer.contains(c) {
                println!("requires {} since it exists in {}", c, self.answer);
                // correct character in wrong position
                self.required.include(c);
                self.space.exclude(i, c);
            } else {
                println!("rejecting {}", c);
                // incorrect character
                self.rejected.include(c);
                for i in 0..5 {
                    self.space.exclude(i, c);
                }
            }
        }
    }

    pub fn matches(&self, word: Word) -> bool {
        let wm = word.charset();

        // ensure we dont have any rejected characters
        if wm.contains_any(self.rejected) {
            return false;
        }

        // ensure we have all required characters
        if !wm.contains_all(self.required) {
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
        assert_eq!(space.matches(Word::new("abcde")), true);
        assert_eq!(space.matches(Word::new("bcdea")), true);
        space.exclude(0, 'a');
        assert_eq!(space.matches(Word::new("abcde")), false);
    }

    #[test]
    fn test_wordspace_only() {
        let mut space = WordSpace::new();
        space.only(0, 'a');
        assert_eq!(space.0[0].includes('a'), true);
        for c in 'b'..='z' {
            assert_eq!(space.0[0].includes(c), false);
        }
        assert_eq!(space.matches(Word::new("abcde")), true);
        assert_eq!(space.matches(Word::new("bbcde")), false);
    }

    #[test]
    fn test_filter() {
        let answer = Word::new("theta");
        let guess1 = Word::new("beast");
        let guess2 = Word::new("tears");
        let guess3 = Word::new("tamed");
        let mut filter = WordFilter::new(answer);

        println!("{:?}", guess1);
        assert_eq!(filter.matches(guess1), true);
        filter.apply(guess1);
        println!("{:?}", filter);
        assert_eq!(filter.matches(guess1), false);
        assert_eq!(filter.matches(answer), true);

        println!("{:?}", guess2);
        filter.apply(guess2);
        println!("{:?}", filter);
        assert_eq!(filter.matches(answer), true);

        println!("{:?}", guess3);
        filter.apply(guess3);
        println!("{:?}", filter);
        assert_eq!(filter.matches(answer), true);

        assert_eq!(filter.matches(answer), true); // no longer matches the answer??
        assert_eq!(filter.matches(Word::new("steal")), false);
        assert_eq!(filter.matches(Word::new("steak")), false);
    }
}
