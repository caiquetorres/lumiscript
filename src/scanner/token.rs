use crate::line_column::LineColumn;

/// `TokenKind` is an enumeration of different token types that can be
/// produced during lexical analysis. Each variant corresponds to a
/// specific keyword, punctuation, or identifier type in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    If,
    Else,
    For,
    Fun,
    Extern,
    Let,
    Ident,
    String,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Eof,
    Semicolon,
    Comma,
    Colon,
    Bad,
    Dot,
    DotDot,
    DotDotEqual,
    Plus,
    Minus,
    MinusGreater,
    Star,
    Slash,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Number,
    Nil,
    Class,
    In,
    True,
    False,
    Return,
    Break,
    Continue,
    While,
    Println,
    Print,
    Const,
    Trait,
    Impl,
    Interrogation,
    Static,
}

/// Represents a span or range in the source code, identified by start and
/// end positions.
///
/// `Span` is a structure used to denote a specific portion of the source
/// code. It includes information about the start and end positions, as
/// well as an optional reference to the source text for that span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub source_text: Option<String>,
    pub start: LineColumn,
    pub end: LineColumn,
}

/// Represents a lexical token in the source code, including its kind and
/// span.
///
/// `Token` is a structure used to represent individual lexical tokens in
/// the source code. It includes information about the token kind (type)
/// and the span (range) in the source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, content: Option<&str>, start: LineColumn, end: LineColumn) -> Self {
        Self {
            kind,
            span: Span {
                source_text: content.map(|v| v.to_owned()),
                start,
                end,
            },
        }
    }
}

#[macro_export]
macro_rules! ident {
    () => {
        TokenKind::Ident
    };
}

#[macro_export]
macro_rules! string {
    () => {
        TokenKind::String
    };
}

#[macro_export]
macro_rules! number {
    () => {
        TokenKind::Number
    };
}

#[macro_export]
macro_rules! token {
    ('(') => {
        TokenKind::LeftParen
    };
    (')') => {
        TokenKind::RightParen
    };
    ('{') => {
        TokenKind::LeftBrace
    };
    ('}') => {
        TokenKind::RightBrace
    };
    (+) => {
        TokenKind::Plus
    };
    (-) => {
        TokenKind::Minus
    };
    (->) => {
        TokenKind::MinusGreater
    };
    (*) => {
        TokenKind::Star
    };
    (/) => {
        TokenKind::Slash
    };
    (:) => {
        TokenKind::Colon
    };
    (.) => {
        TokenKind::Dot
    };
    (..) => {
        TokenKind::DotDot
    };
    (..=) => {
        TokenKind::DotDotEqual
    };
    (,) => {
        TokenKind::Comma
    };
    (;) => {
        TokenKind::Semicolon
    };
    (=) => {
        TokenKind::Equal
    };
    (==) => {
        TokenKind::EqualEqual
    };
    (!) => {
        TokenKind::Bang
    };
    (!=) => {
        TokenKind::BangEqual
    };
    (>) => {
        TokenKind::Greater
    };
    (>=) => {
        TokenKind::GreaterEqual
    };
    (<) => {
        TokenKind::Less
    };
    (<=) => {
        TokenKind::LessEqual
    };
    (?) => {
        TokenKind::Interrogation
    };
    (trait) => {
        TokenKind::Trait
    };
    (impl) => {
        TokenKind::Impl
    };
    (fun) => {
        TokenKind::Fun
    };
    (extern) => {
        TokenKind::Extern
    };
    (static) => {
        TokenKind::Static
    };
    (let) => {
        TokenKind::Let
    };
    (const) => {
        TokenKind::Const
    };
    (if) => {
        TokenKind::If
    };
    (else) => {
        TokenKind::Else
    };
    (nil) => {
        TokenKind::Nil
    };
    (class) => {
        TokenKind::Class
    };
    (for) => {
        TokenKind::For
    };
    (in) => {
        TokenKind::In
    };
    (true) => {
        TokenKind::True
    };
    (false) => {
        TokenKind::False
    };
    (return) => {
        TokenKind::Return
    };
    (break) => {
        TokenKind::Break
    };
    (continue) => {
        TokenKind::Continue
    };
    (while) => {
        TokenKind::While
    };
    (class) => {
        TokenKind::Class
    };
    (println) => {
        TokenKind::Println
    };
    (print) => {
        TokenKind::Print
    };
    (bad) => {
        TokenKind::Bad
    };
    (eof) => {
        TokenKind::Eof
    };
}

pub(crate) use token;
