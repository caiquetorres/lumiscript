use lumi_lxr::token::TokenKind;

use crate::compile_error::CompileError;
use crate::syntax::display_tree::branch;

use super::parse::Parse;
use super::parse::ParseStream;
use super::symbols::Ident;
use crate::syntax::display_tree::DisplayTree;

pub struct Type {
    ident: Ident,
    nullable: bool,
}

impl Type {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn nullable(&self) -> bool {
        self.nullable
    }
}

impl Parse for Type {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Type {
            ident: input.parse()?,
            nullable: {
                if input.peek() == TokenKind::Interrogation {
                    input.next();
                    true
                } else {
                    false
                }
            },
        })
    }
}

impl DisplayTree for Type {
    fn display(&self, layer: usize) {
        branch(
            &format!(
                "Type: {}{}",
                self.ident.source_text(),
                if self.nullable { "?" } else { "" }
            ),
            layer,
        );
    }
}
