use std::{ fmt::{ Display, Formatter, Result } };

use crate::types::*;
use crate::lexer::Location;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    location: Location,
    text: &'static str

}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,              // (
    RightParen,             // )
    LeftBrace,              // {
    RightBrace,             // }
    LeftBracket,            // [
    RightBracket,           // ]

    Let,                    // let
    Fn,                     // fn
    If,                     // if
    Else,                   // else
    Import,                 // import
    From,                   // from
    Return,                 // return
    Extern,                 // extern
    While,                  // while
    Type,                   // type
    Struct,                 // struct
    Trait,                  // trait
    Enum,                   // enum
    New,                    // new
    Delete,                 // delete
    Sizeof,                 // sizeof
    As,                     // as
    Static,                 // static
    Inline,                 // inline
    Abstract,               // abstract
    Mut,                    // mut

    Bang,                   // !
    Equals,                 // =
    Plus,                   // +
    Minus,                  // -
    Star,                   // *
    Slash,                  // /
    Modulo,                 // %

    Smaller,                // <
    Greater,                // >
    Ampersand,              // &
    Pipe,                   // |

    TripleDot,              // ...
    Colon,                  // :
    Semicolon,              // ;
    Dot,                    // .
    Comma,                  // ,
    Question,               // ?

    EqualsEquals,           // ==
    BangEquals,             // !=
    SmallerEquals,          // <=
    GreaterEquals,          // >=
    AmpersandAmpersand,     // &&
    PipePipe,               // ||
    Arrow,                  // =>

    NullLit,
    Identifier(&'static str),
    IntLit(&'static str),
    FloatLit(&'static str),
    StringLit(&'static str),
    Char(&'static str),
    TypeIdentifier(SimpleType),
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::LeftParen => write!(f, "("),
            Self::RightParen => write!(f, ")"),
            Self::LeftBrace => write!(f, "{{"),
            Self::RightBrace => write!(f, "}}"),
            Self::LeftBracket => write!(f, "["),
            Self::RightBracket => write!(f, "]"),

            Self::Let => write!(f, "let"),
            Self::Fn => write!(f, "fn"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Import => write!(f, "import"),
            Self::From => write!(f, "from"),
            Self::Return => write!(f, "return"),
            Self::Extern => write!(f, "extern"),
            Self::While => write!(f, "while"),
            Self::Type => write!(f, "type"),
            Self::Struct => write!(f, "struct"),
            Self::Trait => write!(f, "trait"),
            Self::Enum => write!(f, "enum"),
            Self::New => write!(f, "new"),
            Self::Delete => write!(f, "delete"),
            Self::Sizeof => write!(f, "sizeof"),
            Self::As => write!(f, "as"),
            Self::Static  => write!(f, "static"),
            Self::Inline => write!(f, "inline"),
            Self::Abstract => write!(f, "abstract"),
            Self::Mut => write!(f, "mut"),

            Self::Bang => write!(f, "!"),
            Self::Equals => write!(f, "="),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Modulo => write!(f, "%"),

            Self::Smaller => write!(f, "<"),
            Self::Greater => write!(f, ">"),
            Self::Ampersand => write!(f, "&"),
            Self::Pipe => write!(f, "|"),

            Self::TripleDot => write!(f, "..."),
            Self::Colon => write!(f, ":"),
            Self::Semicolon => write!(f, ";"),
            Self::Dot => write!(f, "."),
            Self::Comma => write!(f, ","),
            Self::Question => write!(f, "?"),

            Self::EqualsEquals => write!(f, "=="),
            Self::BangEquals => write!(f, "!="),
            Self::SmallerEquals => write!(f, "<="),
            Self::GreaterEquals => write!(f, ">="),
            Self::AmpersandAmpersand => write!(f, "&&"),
            Self::PipePipe => write!(f, "||"),
            Self::Arrow => write!(f, "=>"),

            Self::NullLit => write!(f, "null"),
            Self::Identifier(ref ident) => write!(f, "{}", ident),
            Self::IntLit(lit) | Self::FloatLit(lit) => write!(f, "{}", lit),
            Self::StringLit(ref string) => write!(f, "{}", string),
            Self::Char(chr) => write!(f, "'{}'", chr),
            Self::TypeIdentifier(ty) => write!(f, "{}", ty),
        }
    }
}
