use crate::scanner::token::TokenKind;
use crate::syntax::display_tree::branch;
use crate::token;

use super::parse::Parse;
use super::parse::ParseStream;
use super::symbols::colon::Colon;
use super::symbols::ident::Ident;
use crate::syntax::display_tree::DisplayTree;

pub struct Type {
    _colon: Colon,
    pub ident: Ident,
    pub nullable: bool,
}

impl Parse for Type {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(Type {
            _colon: input.parse()?,
            ident: input.parse()?,
            nullable: {
                if input.peek() == token!(?) {
                    input.next();
                    true
                } else {
                    false
                }
            },
        })
    }
}

impl Parse for Option<Type> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        if input.peek() == token!(:) {
            Ok(Some(input.parse()?))
        } else {
            Ok(None)
        }
    }
}

impl DisplayTree for Type {
    fn display(&self, layer: usize) {
        branch(
            &format!(
                "Type: {}{}",
                self.ident.name(),
                if self.nullable { "?" } else { "" }
            ),
            layer,
        );
    }
}
