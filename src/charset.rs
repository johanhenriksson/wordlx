#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Charset(u32);

impl Charset {
    pub fn all() -> Self {
        Charset(0b00000011111111111111111111111111)
    }

    pub fn none() -> Self {
        Charset(0)
    }

    pub fn char(c: char) -> Self {
        if c < 'a' || c > 'z' {
            return Charset::none();
        }
        Charset(1 << (c as u32 - b'a' as u32))
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
        self.0 ^= Self::char(c).0;
        self.clone()
    }

    pub fn include(&mut self, c: char) -> Self {
        self.0 |= Self::char(c).0;
        self.clone()
    }

    pub fn contains(&self, other: Charset) -> bool {
        self.0 & other.0 == other.0
    }

    pub fn includes(&self, c: char) -> bool {
        self.0 & Self::char(c).0 != 0
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
}
