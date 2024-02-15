use lumi_lxr::token::TokenKind;

use crate::compile_error::CompileError;

use super::display_tree::{branch, DisplayTree};
use super::parse::{Parse, ParseStream};

#[macro_export]
macro_rules! syntax_symbol {
    ($struct_name:ident,$kind:expr) => {
        pub struct $struct_name {
            span: lumi_lxr::span::Span,
        }

        impl $struct_name {
            pub fn span(&self) -> &lumi_lxr::span::Span {
                &self.span
            }

            pub fn source_text(&self) -> String {
                self.span.source_text()
            }
        }

        impl crate::syntax::parse::Parse for $struct_name {
            fn parse(
                input: &mut crate::syntax::parse::ParseStream,
            ) -> Result<Self, crate::compile_error::CompileError> {
                Ok($struct_name {
                    span: lumi_lxr::span::Span::from(input.expect($kind)?.span()),
                })
            }
        }
    };
}

syntax_symbol!(Colon, TokenKind::Colon);
syntax_symbol!(Semicolon, TokenKind::Semicolon);
syntax_symbol!(LeftBrace, TokenKind::LeftBrace);
syntax_symbol!(RightBrace, TokenKind::RightBrace);
syntax_symbol!(Equal, TokenKind::Equal);
syntax_symbol!(LeftParen, TokenKind::LeftParen);
syntax_symbol!(RightParen, TokenKind::RightParen);
syntax_symbol!(Arrow, TokenKind::MinusGreater);
syntax_symbol!(Dot, TokenKind::Dot);

syntax_symbol!(Trait, TokenKind::Trait);
syntax_symbol!(Class, TokenKind::Class);
syntax_symbol!(Impl, TokenKind::Impl);

syntax_symbol!(Extern, TokenKind::Extern);

impl Parse for Option<Extern> {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        if input.peek() == TokenKind::Extern {
            Ok(Some(input.parse()?))
        } else {
            Ok(None)
        }
    }
}

syntax_symbol!(Static, TokenKind::Static);

impl Parse for Option<Static> {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        if input.peek() == TokenKind::Static {
            Ok(Some(input.parse()?))
        } else {
            Ok(None)
        }
    }
}

syntax_symbol!(Fun, TokenKind::Fun);
syntax_symbol!(Print, TokenKind::Println);
syntax_symbol!(Let, TokenKind::Let);
syntax_symbol!(Const, TokenKind::Const);
syntax_symbol!(False, TokenKind::False);
syntax_symbol!(True, TokenKind::True);
syntax_symbol!(Nil, TokenKind::Nil);
syntax_symbol!(If, TokenKind::If);
syntax_symbol!(Else, TokenKind::Else);
syntax_symbol!(For, TokenKind::For);
syntax_symbol!(In, TokenKind::In);
syntax_symbol!(While, TokenKind::While);
syntax_symbol!(Return, TokenKind::Return);
syntax_symbol!(Break, TokenKind::Break);
syntax_symbol!(Continue, TokenKind::Continue);

syntax_symbol!(Ident, TokenKind::Ident);

impl DisplayTree for Ident {
    fn display(&self, layer: usize) {
        branch(&format!("Ident: {}", self.source_text()), layer)
    }
}
