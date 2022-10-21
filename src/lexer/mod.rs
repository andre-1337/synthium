pub mod lexer;
pub mod token;

use lexer::*;
use token::*;

pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
        }
    }
}

pub struct Location {
    start: Span,
    end: Span,
}

impl Location {
    pub fn from_span(span: Span) -> Self {
        Self {
            start: Span {
                start: span.start,
                end: span.end,
            },
            end: Span {
                start: span.start,
                end: span.end,
            }
        }
    }

    pub fn from_spans(span1: Span, span2: Span) -> Self {
        Self {
            start: Span {
                start: span1.start,
                end: span1.end,
            },
            end: Span {
                start: span2.start,
                end: span2.end,
            }
        }
    }
}
