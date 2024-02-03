use crate::ident;
use crate::number;

use crate::scanner::token::token;
use crate::scanner::token::TokenKind;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;

use super::binary::ExprBinary;
use super::ident::ExprIdent;
use super::lit::ExprLit;
use super::paren::ExprParen;
use super::unary::ExprUnary;

pub enum Expr {
    Ident(ExprIdent),
    Lit(ExprLit),
    Unary(ExprUnary),
    Binary(ExprBinary),
    Paren(ExprParen),
}

impl Parse for Expr {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        ambiguous_expr(input)
    }
}

impl Parse for Box<Expr> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        ambiguous_expr(input).map(Box::new)
    }
}

impl DisplayTree for Expr {
    fn display(&self, layer: usize) {
        match self {
            Self::Ident(ident) => ident.display(layer),
            Self::Binary(bin) => bin.display(layer),
            Self::Lit(lit) => lit.display(layer),
            Self::Paren(paren) => paren.display(layer),
            Self::Unary(un) => un.display(layer),
        }
    }
}

fn ambiguous_expr(input: &mut ParseStream) -> Result<Expr, String> {
    equality(input)
}

fn equality(input: &mut ParseStream) -> Result<Expr, String> {
    let mut left = comparison(input)?;

    while input.peek() == token!(!=) || input.peek() == token!(==) {
        let operator = input.parse()?;
        let right = comparison(input)?;

        left = Expr::Binary(ExprBinary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        });
    }

    Ok(left)
}

fn comparison(input: &mut ParseStream) -> Result<Expr, String> {
    let mut left: Expr = term(input)?;

    while input.peek() == token!(>)
        || input.peek() == token!(>=)
        || input.peek() == token!(<)
        || input.peek() == token!(<=)
    {
        let operator = input.parse()?;
        let right = term(input)?;

        left = Expr::Binary(ExprBinary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        });
    }

    Ok(left)
}

fn term(input: &mut ParseStream) -> Result<Expr, String> {
    let mut left = factor(input)?;

    while input.peek() == token!(+) || input.peek() == token!(-) {
        let operator = input.parse()?;
        let right = factor(input)?;

        left = Expr::Binary(ExprBinary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        });
    }

    Ok(left)
}

fn factor(input: &mut ParseStream) -> Result<Expr, String> {
    let mut left = unary(input)?;

    while input.peek() == token!(*) || input.peek() == token!(/) {
        let operator = input.parse()?;
        let right = unary(input)?;

        left = Expr::Binary(ExprBinary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        });
    }

    Ok(left)
}

fn unary(input: &mut ParseStream) -> Result<Expr, String> {
    if input.peek() == token!(+) || input.peek() == token!(-) || input.peek() == token!(!) {
        Ok(Expr::Unary(ExprUnary {
            operator: input.parse()?,
            expr: Box::new(primary(input)?),
        }))
    } else {
        primary(input)
    }
}

fn primary(input: &mut ParseStream) -> Result<Expr, String> {
    match input.peek() {
        ident!() => Ok(Expr::Ident(input.parse()?)),
        token!(false) | token!(true) | token!(nil) | number!() => Ok(Expr::Lit(input.parse()?)),
        token!('(') => Ok(Expr::Paren(input.parse()?)),
        _ => {
            let start = input.cur().span.start;
            input.error_reporter().report("Expression expected", start);
            Err("Expression expected".to_owned())
        }
    }
}
