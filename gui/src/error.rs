use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct ParserError(pub String);

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse operation: {}", self.0)
    }
}
