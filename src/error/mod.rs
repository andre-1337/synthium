use std::{ fmt::{ Display, Formatter, Result } };

use crate::lexer::{ Span };

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorType {
    InternalError,
    TypeError,
    LexError,
    ParseError,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::InternalError => write!(f, "[InternalError]"),
            Self::TypeError => write!(f, "[TypeError]"),
            Self::LexError => write!(f, "[LexError]"),
            Self::ParseError => write!(f, "[ParseError]"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Error {
    pub location: Span,
    pub error_type: ErrorType,
    pub message: &'static str,
}

impl Error {
    pub fn new(start: usize, end: usize, error_type: ErrorType, message: &'static str) -> Self {
        Self {
            location: Span {
                start,
                end,
            },
            error_type,
            message,
        }
    }

    #[inline] pub fn get_location(&self) -> Span {
        self.location
    }

    #[inline] pub fn get_error_type(&self) -> ErrorType {
        self.error_type
    }

    #[inline] pub fn get_message(&self) -> String {
        self.message.to_owned()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let msg: String = format!("{} at {} : {}", self.error_type, self.location, self.message);

        write!(f, "{}", msg)
    }
}
