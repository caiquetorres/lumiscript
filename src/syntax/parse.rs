use crate::error_reporter::ErrorReporter;
use crate::scanner::token::Token;
use crate::scanner::token::TokenKind;
use crate::source_code::SourceCode;

pub trait Parse: Sized {
    fn parse(input: &mut ParseStream) -> Result<Self, String>;
}

pub struct ParseStream {
    prev: usize,
    cur: usize,
    tokens: Vec<Token>,
    error_reporter: ErrorReporter,
}

impl ParseStream {
    pub fn new(tokens: Vec<Token>, source_code: SourceCode) -> Self {
        Self {
            prev: 0,
            cur: 0,
            tokens,
            error_reporter: ErrorReporter::new(source_code),
        }
    }

    pub fn error_reporter(&mut self) -> &mut ErrorReporter {
        &mut self.error_reporter
    }

    pub fn expect(&mut self, kind: TokenKind) -> Result<Token, String> {
        let token = self.cur();
        let prev_token = self.prev();

        self.prev = self.cur;
        self.cur += 1;

        if token.kind == kind {
            Ok(token)
        } else {
            let message = format!("Expected '{:?}'", kind);
            self.error_reporter.report(&message, prev_token.span.end);
            Err(message)
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

    pub fn parse<T: Parse>(&mut self) -> Result<T, String> {
        T::parse(self)
    }
}
