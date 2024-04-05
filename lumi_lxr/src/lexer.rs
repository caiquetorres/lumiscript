use std::fmt::Display;

use crate::token::{Token, TokenKind};
use crate::token_stream::TokenStream;
use crate::utils::line_column::LineColumn;
use crate::utils::source_code::SourceCode;
use crate::utils::span::Span;

#[derive(Debug, Clone)]
pub struct LexError {
    message: String,
    span: Span,
    source_code: SourceCode,
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self.span.start().line();
        let column = self.span.start().column();
        let line_content = self.source_code.code().lines().nth(line - 1).unwrap();
        let output = format!(
            "\nCompile Error: {} at Line {} at Column {}\n",
            self.message, line, column,
        ) + &format!("--> {}\n", self.source_code.file_path())
            + &format!("    {} | {}\n", line, line_content)
            + &format!("{}^-- Here", " ".repeat(column + 8));
        write!(f, "{}", output)
    }
}

pub struct Lexer {
    cur_line_column: LineColumn,
    source_code: SourceCode,
    errors: Vec<LexError>,
}

impl Lexer {
    pub fn new(source_code: SourceCode) -> Self {
        Self {
            cur_line_column: LineColumn::default(),
            source_code,
            errors: vec![],
        }
    }

    pub fn tokens(&mut self) -> Result<TokenStream, Vec<LexError>> {
        let mut tokens = vec![];
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        if self.errors.is_empty() {
            Ok(TokenStream::new(&tokens))
        } else {
            Err(self.errors.clone())
        }
    }
}

impl Lexer {
    fn create_token(&self, kind: TokenKind, start: LineColumn, end: LineColumn) -> Token {
        Token::new(kind, start, end, self.source_code.clone())
    }

    fn report_error(&mut self, message: &str, span: Span) {
        self.errors.push(LexError {
            span,
            message: message.to_owned(),
            source_code: self.source_code.clone(),
        });
    }

    fn cur_index(&self) -> usize {
        self.cur_line_column.index()
    }

    fn next_index(&mut self) {
        if self.peek() == '\n' {
            self.cur_line_column.next_line();
        } else {
            self.cur_line_column.next_column();
        }
    }

    fn is_at_end(&self) -> bool {
        self.cur_index() == self.source_code.code().len()
    }

    fn peek(&self) -> char {
        self.source_code
            .code()
            .bytes()
            .nth(self.cur_index())
            .unwrap_or(b'\0') as char
    }

    fn peek2(&self) -> char {
        self.source_code
            .code()
            .bytes()
            .nth(self.cur_index() + 1)
            .unwrap_or(b'\0') as char
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() {
            if self.peek() == '\n' {
                self.cur_line_column.next_line();
            } else {
                self.cur_line_column.next_column();
            }
        }
    }

    fn skip_one_line_comment(&mut self) {
        self.next_index(); // /
        self.next_index(); // /
        while !self.is_at_end() && self.peek() != '\n' {
            self.next_index();
        }
    }

    fn skip_multiline_line_comment(&mut self) {
        let start = self.cur_line_column;
        self.next_index(); // /
        self.next_index(); // *
        while !self.is_at_end() && (self.peek() != '*' || self.peek2() != '/') {
            if self.peek() == '\n' {
                self.cur_line_column.next_line();
            } else {
                self.cur_line_column.next_column();
            }
        }
        let end = self.cur_line_column;
        if self.is_at_end() {
            self.report_error(
                "Expected closing comment",
                Span::new(start, end, self.source_code.clone()),
            );
        }
        self.next_index(); // *
        self.next_index(); // /
    }

    fn next_token(&mut self) -> Option<Token> {
        while !self.is_at_end()
            && ((self.peek() == '/' && self.peek2() == '*')
                || (self.peek() == '/' && self.peek2() == '/')
                || self.peek().is_whitespace())
        {
            if self.peek() == '/' && self.peek2() == '*' {
                self.skip_multiline_line_comment();
            } else if self.peek() == '/' && self.peek2() == '/' {
                self.skip_one_line_comment();
            } else {
                self.skip_whitespace();
            }
        }
        if self.cur_index() > self.source_code.code().len() {
            None
        } else if self.is_at_end() {
            let start = self.cur_line_column;
            self.cur_line_column.next_column();
            let end = start;
            Some(self.create_token(TokenKind::Eof, start, end))
        } else {
            let start = self.cur_line_column;

            match self.peek() {
                ';' => {
                    self.next_index();
                    let end = self.cur_line_column;
                    Some(self.create_token(TokenKind::Semicolon, start, end))
                }
                ',' => {
                    self.next_index();
                    let end = self.cur_line_column;
                    Some(self.create_token(TokenKind::Comma, start, end))
                }
                ':' => {
                    self.next_index();
                    let end = self.cur_line_column;
                    Some(self.create_token(TokenKind::Colon, start, end))
                }
                '(' => {
                    self.next_index();
                    let end = self.cur_line_column;
                    Some(self.create_token(TokenKind::LeftParen, start, end))
                }
                ')' => {
                    self.next_index();
                    let end = self.cur_line_column;
                    Some(self.create_token(TokenKind::RightParen, start, end))
                }
                '{' => {
                    self.next_index();
                    let end = self.cur_line_column;
                    Some(self.create_token(TokenKind::LeftBrace, start, end))
                }
                '}' => {
                    self.next_index();
                    let end = self.cur_line_column;
                    Some(self.create_token(TokenKind::RightBrace, start, end))
                }
                '+' => {
                    self.next_index();
                    let end = self.cur_line_column;
                    Some(self.create_token(TokenKind::Plus, start, end))
                }
                '*' => {
                    self.next_index();
                    let end = self.cur_line_column;
                    Some(self.create_token(TokenKind::Star, start, end))
                }
                '/' => {
                    self.next_index();
                    let end = self.cur_line_column;
                    Some(self.create_token(TokenKind::Slash, start, end))
                }
                '-' => {
                    self.next_index();
                    if self.peek() == '>' {
                        self.next_index();
                        let end = self.cur_line_column;
                        Some(self.create_token(TokenKind::MinusGreater, start, end))
                    } else {
                        let end = self.cur_line_column;
                        Some(self.create_token(TokenKind::Minus, start, end))
                    }
                }
                '=' => {
                    self.next_index();
                    if self.peek() == '=' {
                        self.next_index();
                        let end = self.cur_line_column;
                        Some(self.create_token(TokenKind::EqualEqual, start, end))
                    } else {
                        let end = self.cur_line_column;
                        Some(self.create_token(TokenKind::Equal, start, end))
                    }
                }
                '!' => {
                    self.next_index();
                    if self.peek() == '=' {
                        self.next_index();
                        let end = self.cur_line_column;
                        Some(self.create_token(TokenKind::BangEqual, start, end))
                    } else {
                        let end = self.cur_line_column;
                        Some(self.create_token(TokenKind::Bang, start, end))
                    }
                }
                '.' => {
                    self.next_index();
                    if self.peek() == '.' {
                        self.next_index();
                        if self.peek() == '=' {
                            self.next_index();
                            let end = self.cur_line_column;
                            Some(self.create_token(TokenKind::DotDotEqual, start, end))
                        } else {
                            let end = self.cur_line_column;
                            Some(self.create_token(TokenKind::DotDot, start, end))
                        }
                    } else {
                        let end = self.cur_line_column;
                        Some(self.create_token(TokenKind::Dot, start, end))
                    }
                }
                '>' => {
                    self.next_index();
                    if self.peek() == '=' {
                        self.next_index();
                        let end = self.cur_line_column;
                        Some(self.create_token(TokenKind::Greater, start, end))
                    } else {
                        let end = self.cur_line_column;
                        Some(self.create_token(TokenKind::GreaterEqual, start, end))
                    }
                }
                '<' => {
                    self.next_index();
                    if self.peek() == '=' {
                        self.next_index();
                        let end = self.cur_line_column;
                        Some(self.create_token(TokenKind::Less, start, end))
                    } else {
                        let end = self.cur_line_column;
                        Some(self.create_token(TokenKind::LessEqual, start, end))
                    }
                }
                _ => {
                    if self.peek().is_digit(10) {
                        let start = self.cur_line_column;
                        while self.peek().is_digit(10) {
                            self.next_index();
                        }
                        if self.peek() == '.' && self.peek2().is_digit(10) {
                            self.next_index(); // .
                            while self.peek().is_digit(10) {
                                self.next_index();
                            }
                        }
                        let end = self.cur_line_column;
                        Some(self.create_token(TokenKind::Number, start, end))
                    } else if self.peek().is_alphabetic() || self.peek() == '_' {
                        let start = self.cur_line_column;
                        while self.peek().is_alphanumeric() || self.peek() == '_' {
                            self.next_index();
                        }
                        let end = self.cur_line_column;
                        // TODO: HashMap maybe?
                        let kind = match &self.source_code[start..end] {
                            "trait" => TokenKind::Trait,
                            "class" => TokenKind::Class,
                            "impl" => TokenKind::Impl,
                            "extern" => TokenKind::Extern,
                            "static" => TokenKind::Static,
                            "fun" => TokenKind::Fun,
                            "let" => TokenKind::Let,
                            "const" => TokenKind::Const,
                            "false" => TokenKind::False,
                            "true" => TokenKind::True,
                            "nil" => TokenKind::Nil,
                            "if" => TokenKind::If,
                            "else" => TokenKind::Else,
                            "for" => TokenKind::For,
                            "in" => TokenKind::In,
                            "while" => TokenKind::While,
                            "return" => TokenKind::Return,
                            "break" => TokenKind::Break,
                            "continue" => TokenKind::Continue,
                            "println" => TokenKind::Println,
                            _ => TokenKind::Ident,
                        };

                        Some(self.create_token(kind, start, end))
                    } else {
                        self.next_index();
                        let end = self.cur_line_column;
                        let token = self.create_token(TokenKind::Bad, start, end);
                        self.report_error(
                            &format!("Unexpected token '{}'", token.source_text()),
                            token.span().clone(),
                        );
                        Some(token)
                    }
                }
            }
        }
    }
}
