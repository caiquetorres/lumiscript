use crate::compile_error::CompileError;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::symbols::LeftBrace;
use crate::syntax::symbols::RightBrace;
use crate::syntax::symbols::Ident;
use crate::syntax::symbols::Trait;

use super::fun::StmtFun;

pub struct StmtTrait {
    _trait: Trait,
    ident: Ident,
    _left_brace: LeftBrace,
    methods: Vec<StmtFun>,
    _right_brace: RightBrace,
}

impl StmtTrait {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }
}

impl Parse for StmtTrait {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(StmtTrait {
            _trait: input.parse()?,
            ident: input.parse()?,
            _left_brace: input.parse()?,
            methods: input.parse()?,
            _right_brace: input.parse()?,
        })
    }
}

impl DisplayTree for StmtTrait {
    fn display(&self, layer: usize) {
        branch("TraitStmt", layer);
        self.ident.display(layer + 1);
        self.methods.display(layer + 1);
    }
}
