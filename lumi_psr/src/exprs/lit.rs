use lumi_lxr::span::Span;

use crate::display_tree::branch;
use crate::display_tree::DisplayTree;

#[derive(Debug)]
pub enum LitExpr {
    Num { span: Span },
    Bool { span: Span },
    Nil { span: Span },
}

impl LitExpr {
    pub(crate) fn num(span: &Span) -> Self {
        Self::Num {
            span: Span::from(span),
        }
    }

    pub fn bool(span: &Span) -> Self {
        Self::Bool {
            span: Span::from(span),
        }
    }

    pub fn nil(span: &Span) -> Self {
        Self::Nil {
            span: Span::from(span),
        }
    }

    pub fn span(&self) -> &Span {
        match self {
            Self::Num { span } => span,
            Self::Bool { span } => span,
            Self::Nil { span } => span,
        }
    }
}

impl DisplayTree for LitExpr {
    fn display(&self, layer: usize) {
        branch(&format!("ExprLit: {}", self.span().source_text()), layer)
    }
}
