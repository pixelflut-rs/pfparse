use nom;
use std::fmt;

pub struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ParseError {
    pub fn new(err: nom::Err<nom::error::Error<&str>>) -> Self {
        return ParseError {
            message: format!("Parse error: {}", err),
        };
    }
}
