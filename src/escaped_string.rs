use parser_lib::common::{EscapeStringError, ParsingError};

pub struct InterpretEscapedString<'a> {
    s: std::str::Chars<'a>,
}

impl<'a> InterpretEscapedString<'a> {
    pub fn new(s: &'a str) -> Self {
        InterpretEscapedString { s: s.chars() }
    }
}

impl Iterator for InterpretEscapedString<'_> {
    type Item = Result<char, ParsingError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.s.next().map(|c| match c {
            '\\' => match self.s.next() {
                None => Err(ParsingError::EscapeStringError(EscapeStringError::EscapeAtEndOfString)),
                Some('n') => Ok('\n'),
                Some('\\') => Ok('\\'),
                Some('t') => Ok('\t'),
                Some('"') => Ok('"'),
                Some(c) => Err(ParsingError::EscapeStringError(EscapeStringError::UnrecognizedEscapedChar(c))),
            },
            c => Ok(c),
        })
    }
}
