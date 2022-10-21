use std::{ fmt::{ Display, Formatter, Result } };

pub mod lexer;
pub mod token;

use lexer::*;
use token::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            line,
            column
        }
    }

    #[inline] pub fn get_line(&self) -> usize {
        self.line
    }

    #[inline] pub fn get_column(&self) -> usize {
        self.column
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
