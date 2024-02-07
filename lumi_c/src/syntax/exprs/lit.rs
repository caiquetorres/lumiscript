use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::span::Span;

pub enum ExprLit {
    Num { span: Span },
    Bool { span: Span },
    Nil { span: Span },
}

impl ExprLit {
    pub fn name(&self) -> String {
        match self {
            Self::Num { span } => span.source_text.clone(),
            Self::Bool { span } => span.source_text.clone(),
            Self::Nil { span } => span.source_text.clone(),
        }
    }
}

impl DisplayTree for ExprLit {
    fn display(&self, layer: usize) {
        branch(&format!("ExprLit: {}", self.name()), layer)
    }
}