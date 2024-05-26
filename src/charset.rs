#[derive(Clone, Copy, PartialEq, Eq)]
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

    pub fn from_str(chars: &str) -> Self {
        let mut set = Charset::none();
        for c in chars.chars() {
            set.include(c);
        }
        set
    }

    pub fn exclude(&mut self, c: char) -> Self {
        self.0 &= Self::char(c).inverse().0;
        self.clone()
    }

    pub fn include(&mut self, c: char) -> Self {
        self.0 |= Self::char(c).0;
        self.clone()
    }

    pub fn inverse(&self) -> Self {
        Charset(self.0 ^ Self::all().0)
    }

    pub fn contains_all(&self, other: Charset) -> bool {
        self.0 & other.0 == other.0
    }

    pub fn contains_any(&self, other: Charset) -> bool {
        self.0 & other.0 > 0
    }

    pub fn includes(&self, c: char) -> bool {
        self.0 & Self::char(c).0 != 0
    }
}

impl std::fmt::Display for Charset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars = vec![];
        for i in 0..26 {
            if self.0 & (1 << i) != 0 {
                chars.push((b'a' + i as u8) as char);
            }
        }
        write!(f, "{}", chars.iter().collect::<String>())
    }
}

impl std::fmt::Debug for Charset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_charset() {
        let set = Charset::char('a');
        assert_ne!(set, Charset::none());
    }

    #[test]
    fn test_charset_include_exclude() {
        let mut set = Charset::none();
        assert_eq!(set.includes('a'), false);
        set.include('a');
        assert_eq!(set.includes('a'), true);
        set.exclude('a');
        assert_eq!(set.includes('a'), false);
    }

    #[test]
    fn test_charset_inverse() {
        let set = Charset::from_str("abc").inverse();
        assert_eq!(set.includes('a'), false);
        assert_eq!(set.includes('d'), true);

        assert_eq!(Charset::all().inverse(), Charset::none());
        assert_eq!(Charset::none().inverse(), Charset::all());
    }

    #[test]
    fn test_charset_contains_all() {
        let set = Charset::from_str("abc");
        assert_eq!(set.contains_all(Charset::from_str("ab")), true);
        assert_eq!(set.contains_all(Charset::from_str("bc")), true);
        assert_eq!(set.contains_all(Charset::from_str("abc")), true);
        assert_eq!(set.contains_all(Charset::from_str("abcd")), false);
    }

    #[test]
    fn test_charset_contains_any() {
        let set = Charset::from_str("abc");
        assert_eq!(set.contains_any(Charset::from_str("ab")), true);
        assert_eq!(set.contains_any(Charset::from_str("abcd")), true);
        assert_eq!(set.contains_any(Charset::from_str("def")), false);
    }
}
