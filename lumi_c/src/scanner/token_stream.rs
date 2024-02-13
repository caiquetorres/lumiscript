use lumi_core::line_column::LineColumn;
use lumi_core::source_code::SourceCode;

use crate::compile_error::CompileError;
use crate::ident;
use crate::number;
use crate::string;
use crate::token;

use super::token::Token;
use super::token::TokenKind;

/// Represents a lexer responsible for tokenizing the source code.
pub struct TokenStream {
    cur: usize,
    line_column: LineColumn,
    source_code: SourceCode,
    errors: Vec<CompileError>,
}

impl TokenStream {
    pub fn tokens(&mut self) -> Result<Vec<Token>, Vec<CompileError>> {
        let mut tokens = vec![];

        while let Some(token) = self.next() {
            tokens.push(token);
        }

        if self.errors.is_empty() {
            Ok(tokens)
        } else {
            Err(self.errors.clone())
        }
    }

    pub fn new(source_code: SourceCode) -> Self {
        Self {
            cur: 0,
            source_code: source_code.clone(),
            line_column: LineColumn::default(),
            errors: vec![],
        }
    }

    fn consume(&mut self) {
        self.cur += 1;
        self.line_column.next_column();
    }

    fn at_end(&self) -> bool {
        self.cur == self.source_code.code().len()
    }

    fn peek(&self) -> char {
        self.source_code
            .code()
            .chars()
            .nth(self.cur)
            .unwrap_or('\0')
    }

    fn peek2(&self) -> char {
        self.source_code
            .code()
            .chars()
            .nth(self.cur + 1)
            .unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() {
            if self.peek() == '\n' {
                self.line_column.next_line();
            } else {
                self.line_column.next_column();
            }

            self.cur += 1;
        }

        // REVIEW: We should improve this recursion. Maybe removing it.

        if self.peek() == '/' && self.peek2() == '/' {
            self.skip_one_line_comment();
        }

        if self.peek() == '/' && self.peek2() == '*' {
            self.skip_multiline_line_comment();
        }
    }

    fn skip_one_line_comment(&mut self) {
        self.consume(); // /
        self.consume(); // /

        while !self.at_end() && self.peek() != '\n' {
            self.consume();
        }

        if self.peek().is_whitespace() {
            self.skip_whitespace();
        }
    }

    fn skip_multiline_line_comment(&mut self) {
        self.consume(); // /
        self.consume(); // *

        while !self.at_end() && (self.peek() != '*' || self.peek2() != '/') {
            if self.peek() == '\n' {
                self.line_column.next_line();
            } else {
                self.line_column.next_column();
            }

            self.consume();
        }

        self.consume(); // *
        self.consume(); // /

        if self.peek().is_whitespace() {
            self.skip_whitespace();
        }
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peek().is_whitespace() {
            self.skip_whitespace();
        }

        if self.peek() == '/' && self.peek2() == '/' {
            self.skip_one_line_comment();
        }

        if self.peek() == '/' && self.peek2() == '*' {
            self.skip_multiline_line_comment();
        }

        if self.cur > self.source_code.code().len() {
            None
        } else if self.at_end() {
            let start = self.line_column;
            let end = self.line_column;

            self.cur += 1;
            Some(Token::new(token!(eof), None, start, end))
        } else {
            let start = self.line_column;

            match self.peek() {
                '"' => {
                    let s = self.cur;
                    self.consume(); // "

                    while !self.at_end() && self.peek() != '"' {
                        self.consume();
                    }

                    if self.at_end() {
                        Some(Token::new(
                            token!(bad),
                            Some(&self.source_code.code()[s..self.cur]),
                            start,
                            self.line_column,
                        ))
                    } else {
                        self.consume(); // "
                        Some(Token::new(
                            string!(),
                            Some(&self.source_code.code()[s + 1..self.cur - 1]),
                            start,
                            self.line_column,
                        ))
                    }
                }
                ';' => {
                    self.consume();
                    Some(Token::new(token!(;), Some(";"), start, self.line_column))
                }
                ',' => {
                    self.consume();
                    Some(Token::new(token!(,), Some(","), start, self.line_column))
                }
                '+' => {
                    self.consume();
                    Some(Token::new(token!(+), Some("+"), start, self.line_column))
                }
                '-' => {
                    self.consume();
                    if self.peek() == '>' {
                        self.consume();
                        Some(Token::new(token!(->), Some("->"), start, self.line_column))
                    } else {
                        Some(Token::new(token!(-), Some("-"), start, self.line_column))
                    }
                }
                '*' => {
                    self.consume();
                    Some(Token::new(token!(*), Some("*"), start, self.line_column))
                }
                '/' => {
                    self.consume();
                    Some(Token::new(token!(/), Some("/"), start, self.line_column))
                }
                '(' => {
                    self.consume();
                    Some(Token::new(token!('('), Some("("), start, self.line_column))
                }
                '{' => {
                    self.consume();
                    Some(Token::new(token!('{'), Some("{"), start, self.line_column))
                }
                '}' => {
                    self.consume();
                    Some(Token::new(token!('}'), Some("}"), start, self.line_column))
                }
                ')' => {
                    self.consume();
                    Some(Token::new(token!(')'), Some(")"), start, self.line_column))
                }
                '.' => {
                    self.consume();
                    if self.peek() == '.' {
                        self.consume();

                        if self.peek() == '=' {
                            self.consume();
                            Some(Token::new(
                                token!(..=),
                                Some("..="),
                                start,
                                self.line_column,
                            ))
                        } else {
                            Some(Token::new(token!(..), Some(".."), start, self.line_column))
                        }
                    } else {
                        Some(Token::new(token!(.), Some("."), start, self.line_column))
                    }
                }
                ':' => {
                    self.consume();
                    Some(Token::new(token!(:), Some(":"), start, self.line_column))
                }
                '?' => {
                    self.consume();
                    Some(Token::new(token!(?), Some("?"), start, self.line_column))
                }
                '=' => {
                    self.consume();
                    if self.peek() == '=' {
                        self.consume();
                        Some(Token::new(token!(==), Some("=="), start, self.line_column))
                    } else {
                        Some(Token::new(token!(=), Some("="), start, self.line_column))
                    }
                }
                '!' => {
                    self.consume();
                    if self.peek() == '=' {
                        self.consume();
                        Some(Token::new(token!(!=), Some("!="), start, self.line_column))
                    } else {
                        Some(Token::new(token!(!), Some("!"), start, self.line_column))
                    }
                }
                '>' => {
                    self.consume();
                    if self.peek() == '=' {
                        self.consume();
                        Some(Token::new(token!(>=), Some(">="), start, self.line_column))
                    } else {
                        Some(Token::new(token!(>), Some(">"), start, self.line_column))
                    }
                }
                '<' => {
                    self.consume();
                    if self.peek() == '=' {
                        self.consume();
                        Some(Token::new(token!(<=), Some("<="), start, self.line_column))
                    } else {
                        Some(Token::new(token!(<), Some("<"), start, self.line_column))
                    }
                }
                _ => {
                    if self.peek().is_alphabetic() || self.peek() == '_' {
                        let s = self.cur;

                        while self.peek().is_alphanumeric() || self.peek() == '_' {
                            self.consume();
                        }

                        let e = self.cur;

                        let text = &self.source_code.code()[s..e];

                        return match text {
                            "fun" => Some(Token::new(
                                token!(fun),
                                Some("fun"),
                                start,
                                self.line_column,
                            )),
                            "extern" => Some(Token::new(
                                token!(extern),
                                Some("extern"),
                                start,
                                self.line_column,
                            )),
                            "static" => Some(Token::new(
                                token!(static),
                                Some("static"),
                                start,
                                self.line_column,
                            )),
                            "let" => Some(Token::new(
                                token!(let),
                                Some("let"),
                                start,
                                self.line_column,
                            )),
                            "const" => Some(Token::new(
                                token!(const),
                                Some("const"),
                                start,
                                self.line_column,
                            )),
                            "if" => {
                                Some(Token::new(token!(if), Some("if"), start, self.line_column))
                            }
                            "else" => Some(Token::new(
                                token!(else),
                                Some("else"),
                                start,
                                self.line_column,
                            )),
                            "nil" => Some(Token::new(
                                token!(nil),
                                Some("nil"),
                                start,
                                self.line_column,
                            )),
                            "class" => Some(Token::new(
                                token!(class),
                                Some("class"),
                                start,
                                self.line_column,
                            )),
                            "for" => Some(Token::new(
                                token!(for),
                                Some("for"),
                                start,
                                self.line_column,
                            )),
                            "in" => {
                                Some(Token::new(token!(in), Some("in"), start, self.line_column))
                            }
                            "while" => Some(Token::new(
                                token!(while),
                                Some("while"),
                                start,
                                self.line_column,
                            )),
                            "true" => Some(Token::new(
                                token!(true),
                                Some("true"),
                                start,
                                self.line_column,
                            )),
                            "false" => Some(Token::new(
                                token!(false),
                                Some("false"),
                                start,
                                self.line_column,
                            )),
                            "return" => Some(Token::new(
                                token!(return),
                                Some("return"),
                                start,
                                self.line_column,
                            )),
                            "println" => Some(Token::new(
                                token!(println),
                                Some("println"),
                                start,
                                self.line_column,
                            )),
                            "print" => Some(Token::new(
                                token!(print),
                                Some("print"),
                                start,
                                self.line_column,
                            )),
                            "impl" => Some(Token::new(
                                token!(impl),
                                Some("impl"),
                                start,
                                self.line_column,
                            )),
                            "break" => Some(Token::new(
                                token!(break),
                                Some("break"),
                                start,
                                self.line_column,
                            )),
                            "continue" => Some(Token::new(
                                token!(continue),
                                Some("continue"),
                                start,
                                self.line_column,
                            )),
                            "trait" => Some(Token::new(
                                token!(trait),
                                Some("trait"),
                                start,
                                self.line_column,
                            )),
                            _ => Some(Token::new(ident!(), Some(text), start, self.line_column)),
                        };
                    };

                    if self.peek().is_digit(10) {
                        let s = self.cur;

                        while self.peek().is_digit(10) {
                            self.consume();
                        }

                        if self.peek() == '.' && self.peek2().is_digit(10) {
                            self.consume(); // .

                            while self.peek().is_digit(10) {
                                self.consume();
                            }
                        }

                        Some(Token::new(
                            number!(),
                            Some(&self.source_code.code()[s..self.cur]),
                            start,
                            self.line_column,
                        ))
                    } else {
                        self.consume();

                        let token_value = &self.source_code.code()[self.cur - 1..self.cur];

                        self.errors.push(CompileError::new(
                            &format!("Unexpected token \"{}\"", token_value),
                            start,
                            self.source_code.clone(),
                        ));

                        Some(Token::new(
                            token!(bad),
                            Some(token_value),
                            start,
                            self.line_column,
                        ))
                    }
                }
            }
        }
    }
}
