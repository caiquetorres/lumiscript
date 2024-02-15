use std::fmt::Debug;

use crate::utils::line_column::LineColumn;
use crate::utils::source_code::SourceCode;
use crate::utils::span::Span;

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
    Const,
    Trait,
    Impl,
    Interrogation,
    Static,
}

#[derive(Clone)]
pub struct Token {
    span: Span,
    kind: TokenKind,
}

impl Token {
    pub fn new(
        kind: TokenKind,
        start: LineColumn,
        end: LineColumn,
        source_code: SourceCode,
    ) -> Self {
        Self {
            kind,
            span: Span::new(start, end, source_code),
        }
    }

    pub fn source_text(&self) -> String {
        self.span.source_text()
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("kind", &self.kind)
            .field("value", &self.span.source_text())
            .finish()
    }
}
