use std::{ fmt::{ Display, Formatter, Result }, borrow::{ Borrow }, hash::{ Hash, Hasher } };

pub mod lexer;
pub mod token;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end
        }
    }

    #[inline] pub fn get_start(&self) -> usize {
        self.start
    }

    #[inline] pub fn get_end(&self) -> usize {
        self.end
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}:{}", self.start, self.end)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Spanned<T> {
    pub span: Span,
    pub node: T,
}

impl<T> Spanned<T> {
    pub fn new(start: usize, end: usize, node: T) -> Self {
        let span = Span { start, end };

        Spanned {
            span,
            node,
        }
    }

    pub fn from_span(span: Span, node: T) -> Self {
        Self {
            span,
            node,
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Source {
    pub name: String,
    pub code: String,
}

impl Source {
    pub fn new(name: &str, code: &str) -> Self {
        Self {
            name: name.to_owned(),
            code: code.to_owned(),
        }
    }

    pub fn slice(&self, location: Span) -> &str {
        &self.code[ location.start ..= location.end ]
    }
}

impl Hash for Source {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Borrow<str> for Source {
    fn borrow(&self) -> &str {
        self.name.as_str()
    }
}
