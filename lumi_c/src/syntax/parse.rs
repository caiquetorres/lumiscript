use lumi_core::source_code::SourceCode;

use crate::compile_error::CompileError;
use crate::scanner::token::Token;
use crate::scanner::token::TokenKind;

pub trait Parse: Sized {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError>;
}

pub struct ParseStream {
    prev: usize,
    cur: usize,
    tokens: Vec<Token>,
    source_code: SourceCode,
}

impl ParseStream {
    pub fn new(tokens: Vec<Token>, source_code: SourceCode) -> Self {
        Self {
            prev: 0,
            cur: 0,
            tokens,
            source_code,
        }
    }

    pub fn source_code(&self) -> SourceCode {
        self.source_code.clone()
    }

    pub fn expect(&mut self, kind: TokenKind) -> Result<Token, CompileError> {
        let token = self.cur();
        let prev_token = self.prev();

        self.prev = self.cur;
        self.cur += 1;

        if token.kind == kind {
            Ok(token)
        } else {
            Err(CompileError::new(
                &format!("Expected '{:?}'", kind),
                prev_token.span.end,
                self.source_code.clone(),
            ))
        }
    }

    pub fn prev(&self) -> Token {
        self.tokens[self.prev].clone()
    }

    pub fn cur(&self) -> Token {
        self.tokens[self.cur].clone()
    }

    pub fn next(&mut self) -> Token {
        let token = self.tokens[self.cur].clone();

        self.prev = self.cur;
        self.cur += 1;

        token
    }

    pub fn peek(&self) -> TokenKind {
        self.tokens[self.cur].kind
    }

    pub fn peek2(&self) -> TokenKind {
        self.tokens[self.cur + 1].kind
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T, CompileError> {
        T::parse(self)
    }
}
