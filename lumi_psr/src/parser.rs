use std::fmt::Display;

use lumi_lxr::source_code::SourceCode;
use lumi_lxr::span::Span;
use lumi_lxr::token::{Token, TokenKind};
use lumi_lxr::token_stream::TokenStream;

use crate::parse::Parse;

pub struct ParseError {
    pub(crate) message: String,
    pub(crate) span: Span,
    pub(crate) source_code: SourceCode,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self.span.start().line();
        let column = self.span.start().column();
        let line_content = self.source_code.code().lines().nth(line - 1).unwrap_or("");
        let output = format!(
            "\nCompile Error: {} at Line {} at Column {}\n",
            self.message, line, column,
        ) + &format!("--> {}\n", self.source_code.file_path())
            + &format!("    {} | {}\n", line, line_content)
            + &format!("{}^-- Here", " ".repeat(column + 7));
        write!(f, "{}", output)
    }
}

pub struct ParseStream {
    index: usize,
    stream: TokenStream,
}

impl ParseStream {
    pub fn parse<T: Parse>(&mut self) -> Result<T, ParseError> {
        T::parse(self)
    }
}

impl ParseStream {
    pub fn new(stream: TokenStream) -> Self {
        Self { index: 0, stream }
    }

    pub(crate) fn peek(&self) -> &Token {
        &self.stream[self.index]
    }

    pub(crate) fn peek2(&self) -> &Token {
        &self.stream[self.index + 1]
    }

    pub(crate) fn next(&mut self) -> &Token {
        let min = 0;
        let max = self.stream.iter().len() - 1;
        self.index = (self.index + 1).clamp(min, max);
        &self.stream[self.index - 1]
    }

    pub(crate) fn expect(&mut self, kind: TokenKind) -> Result<&Token, ParseError> {
        let token = &self.stream[self.index];
        self.index += 1;
        if token.kind() == kind {
            Ok(token)
        } else {
            Err(ParseError {
                message: format!("Expected '{:?}'", kind),
                source_code: token.span().source_code().clone(),
                span: token.span().clone(),
            })
        }
    }
}
