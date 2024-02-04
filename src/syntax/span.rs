use crate::{line_column::LineColumn, scanner::token::Token};

#[derive(Debug)]
pub struct Span {
    pub source_text: String,
    pub start: LineColumn,
    pub end: LineColumn,
}

impl Span {
    pub fn from_token(token: Token) -> Self {
        Self {
            start: token.span.start,
            end: token.span.end,
            source_text: token.span.source_text.unwrap(),
        }
    }
}
