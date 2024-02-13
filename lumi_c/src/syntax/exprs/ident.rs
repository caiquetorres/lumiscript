use crate::compile_error::CompileError;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::symbols::ident::Ident;

pub struct ExprIdent {
    pub ident: Ident,
}

impl Parse for ExprIdent {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(ExprIdent {
            ident: input.parse()?,
        })
    }
}

impl DisplayTree for ExprIdent {
    fn display(&self, layer: usize) {
        branch(&format!("IdentExpr: {}", self.ident.name()), layer);
    }
}
