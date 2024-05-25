use crate::word::Word;

fn char_mask(c: char) -> u32 {
    1 << (c as u32 - b'a' as u32)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Charset(u32);

impl Charset {
    pub fn all() -> Self {
        Charset(0b00000011111111111111111111111111)
    }

    pub fn none() -> Self {
        Charset(0)
    }

    #[allow(dead_code)]
    pub fn from_str(chars: &str) -> Self {
        let mut set = Charset::none();
        for c in chars.chars() {
            set.include(c);
        }
        set
    }

    pub fn exclude(&mut self, c: char) -> Self {
        self.0 ^= char_mask(c);
        self.clone()
    }

    pub fn include(&mut self, c: char) -> Self {
        self.0 |= char_mask(c);
        self.clone()
    }

    pub fn inverse(&self) -> Self {
        return Charset(self.0 ^ Self::all().0);
    }

    pub fn contains(&self, other: Charset) -> bool {
        self.0 & other.0 == other.0
    }

    pub fn intersects(&self, other: Charset) -> bool {
        self.0 & other.0 != 0
    }

    pub fn includes(&self, c: char) -> bool {
        self.0 & char_mask(c) != 0
    }
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_include_exclude() {
        let mut set = Charset::none();
        assert_eq!(set.includes('a'), false);
        set.include('a');
        assert_eq!(set.includes('a'), true);
        set.exclude('a');
        assert_eq!(set.includes('a'), false);
    }

    #[test]
    fn test_contains() {
        let set = Charset::from_str("abc");
        assert_eq!(set.contains(Charset::from_str("ab")), true);
        assert_eq!(set.contains(Charset::from_str("cd")), false);
    }

    #[test]
    fn test_inverse() {
        let set = Charset::from_str("abc");
        let inverse = set.inverse();
        assert_eq!(inverse.includes('a'), false);
        assert_eq!(inverse.includes('b'), false);
        assert_eq!(inverse.includes('c'), false);
        assert_eq!(inverse.includes('d'), true);
    }

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
}
