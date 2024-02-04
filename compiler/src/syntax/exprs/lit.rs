use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;

pub struct ExprLit {
    span: Span,
}

impl ExprLit {
    pub fn name(&self) -> String {
        self.span.source_text.clone()
    }
}

impl Parse for ExprLit {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(ExprLit {
            span: Span::from_token(input.next()),
        })
    }
}

impl DisplayTree for ExprLit {
    fn display(&self, layer: usize) {
        branch(&format!("ExprLit: {}", self.name()), layer)
    }
}
