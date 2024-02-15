use lumi_lxr::span::Span;
use lumi_lxr::token::TokenKind;

use crate::display_tree::branch;
use crate::display_tree::DisplayTree;
use crate::parse::Parse;
use crate::parser::ParseError;
use crate::parser::ParseStream;
use crate::symbols::LeftParen;
use crate::symbols::RightParen;

use super::Expr;

impl Parse for Vec<Expr> {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        if input.peek().kind() == TokenKind::RightParen {
            Ok(vec![])
        } else {
            let mut args = vec![];
            args.push(input.parse()?);
            while input.peek().kind() != TokenKind::RightParen {
                input.expect(TokenKind::Comma)?;
                args.push(input.parse()?);
            }
            Ok(args)
        }
    }
}

impl DisplayTree for Vec<Expr> {
    fn display(&self, layer: usize) {
        for expr in self {
            expr.display(layer);
        }
    }
}

pub struct CallExpr {
    span: Span,
    callee: Box<Expr>,
    args: Vec<Expr>,
}

impl CallExpr {
    pub(crate) fn new(
        callee: Expr,
        _left_paren: LeftParen,
        args: Vec<Expr>,
        right_paren: RightParen,
    ) -> Self {
        Self {
            span: Span::range(callee.span(), right_paren.span()),
            args,
            callee: Box::new(callee),
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl DisplayTree for CallExpr {
    fn display(&self, layer: usize) {
        branch("CallExpr", layer);
        self.callee.display(layer + 1);
        if !self.args.is_empty() {
            branch("Args", layer + 1);
            self.args.display(layer + 2);
        }
    }
}
