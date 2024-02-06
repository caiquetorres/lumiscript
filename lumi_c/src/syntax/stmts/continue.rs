use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::{Parse, ParseStream};
use crate::syntax::symbols::r#continue::Continue;
use crate::syntax::symbols::semicolon::Semicolon;

pub struct StmtContinue {
    _return: Continue,
    _semicolon: Semicolon,
}

impl Parse for StmtContinue {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(StmtContinue {
            _return: input.parse()?,
            _semicolon: input.parse()?,
        })
    }
}

impl DisplayTree for StmtContinue {
    fn display(&self, layer: usize) {
        branch("ContinueStmt", layer);
    }
}
