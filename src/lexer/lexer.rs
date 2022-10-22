use std::{ iter::{ Peekable }, str::{ CharIndices }, fmt::{ Display, Formatter } };

use super::{ Span, Spanned, Source };
use crate::{ error::{ * }, types::{ * }, lexer::{ token::{ Token, TokenType } } };

type Scanned = Result<Spanned<Token>, Spanned<ErrorType>>;

pub trait Scanner: Iterator<Item = Scanned> {
    fn source(&self) -> &'static Source;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InputPosition {
    pos: usize,
    val: char,
}

impl InputPosition {
    fn new_opt(value: Option<(usize, char)>) -> Option<Self> {
        let (pos, val) = value?;

        Some(InputPosition {
            pos,
            val,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Lexer {
    source: &'static Source,
    src: &'static str,
    chars: Peekable<CharIndices<'static>>,
    current: Option<InputPosition>,
    previous: Option<char>,
}

impl Lexer {
    pub fn new(source: &'static Source) -> Self {
        let src = &source.code;
        let mut chars = src.char_indices().peekable();

        Lexer {
            source,
            src,
            current: InputPosition::new_opt(chars.next()),
            chars,
            previous: None
        }
    }

    fn pos(&self) -> usize {
        if let Some(InputPosition { pos, .. }) = self.current {
            return pos;
        }

        self.src.len()
    }

    fn slice(&self, start: usize, end: usize) -> &'static str {
        let end = if end > self.src.len() {
            self.src.len()
        } else {
            end
        };

        &self.src[ start .. end ]
    }

    fn spanned<T>(&self, start: usize, t: T) -> Spanned<T> {
        Spanned::new(start, self.pos() - self.previous.map_or(0, char::len_utf8), t)
    }


}
