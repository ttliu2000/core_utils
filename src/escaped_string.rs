#[derive(Debug, Clone, PartialEq, Hash)]
pub enum EscapeStringError {
    EscapeAtEndOfString,
    UnrecognizedEscapedChar(char),
}

impl std::fmt::Display for EscapeStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EscapeAtEndOfString => write!(f, "Escape character at the end of the string"),
            Self::UnrecognizedEscapedChar(c) => write!(f, "Unrecognized escaped char: '{}'", c),
        }
    }
}

impl std::error::Error for EscapeStringError {}

pub struct InterpretEscapedString<'a> {
    s: std::str::Chars<'a>,
}

impl<'a> InterpretEscapedString<'a> {
    pub fn new(s: &'a str) -> Self {
        InterpretEscapedString { s: s.chars() }
    }
}

impl Iterator for InterpretEscapedString<'_> {
    type Item = Result<char, EscapeStringError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.s.next().map(|c| match c {
            '\\' => match self.s.next() {
                None => Err(EscapeStringError::EscapeAtEndOfString),
                Some('n') => Ok('\n'),
                Some('\\') => Ok('\\'),
                Some('t') => Ok('\t'),
                Some('"') => Ok('"'),
                Some(c) => Err(EscapeStringError::UnrecognizedEscapedChar(c)),
            },
            c => Ok(c),
        })
    }
}
