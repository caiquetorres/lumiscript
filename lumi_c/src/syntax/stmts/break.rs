use crate::compile_error::CompileError;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::symbols::r#break::Break;
use crate::syntax::symbols::semicolon::Semicolon;

pub struct StmtBreak {
    _return: Break,
    _semicolon: Semicolon,
}

impl Parse for StmtBreak {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(StmtBreak {
            _return: input.parse()?,
            _semicolon: input.parse()?,
        })
    }
}

impl DisplayTree for StmtBreak {
    fn display(&self, layer: usize) {
        branch("BreakStmt", layer);
    }
}
