mod sealed {

    pub trait Sealed {}

    impl<'a> Sealed for &'a str {}

    impl Sealed for char {}
}

pub trait StringExt: sealed::Sealed {
    fn is_ascii_whitespace(&self) -> bool;
    fn is_ascii_punctuation(&self) -> bool;
    fn is_whitespace(&self) -> bool;

    fn is_digit(&self) -> bool;
}

impl<'a> StringExt for &'a str {
    fn is_ascii_punctuation(&self) -> bool {
        self.chars().all(|m| m.is_ascii_punctuation())
    }

    fn is_ascii_whitespace(&self) -> bool {
        self.chars().all(|m| m.is_ascii_whitespace())
    }

    fn is_whitespace(&self) -> bool {
        self.chars().all(|m| m.is_whitespace())
    }

    fn is_digit(&self) -> bool {
        self.chars().all(|m| m.is_ascii_digit())
    }
}

impl StringExt for char {
    fn is_ascii_whitespace(&self) -> bool {
        (*self).is_ascii_whitespace()
    }

    fn is_ascii_punctuation(&self) -> bool {
        (*self).is_ascii_punctuation()
    }

    fn is_whitespace(&self) -> bool {
        (*self).is_whitespace()
    }

    fn is_digit(&self) -> bool {
        (*self).is_ascii_digit()
    }
}
