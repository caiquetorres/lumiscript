use crate::ident;
use crate::number;

use crate::scanner::token::token;
use crate::scanner::token::Token;
use crate::scanner::token::TokenKind;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::symbols::ident::Ident;

pub struct ExprIdent {
    ident: Ident,
}

impl Parse for ExprIdent {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(ExprIdent {
            ident: input.parse()?,
        })
    }
}

impl DisplayTree for ExprIdent {
    fn display(&self, layer: usize) {
        branch("IdentExpr", layer);
        self.ident.display(layer + 1);
    }
}

pub struct ExprLit {
    pub value: Token,
}

impl DisplayTree for ExprLit {
    fn display(&self, layer: usize) {
        if let Some(literal) = &self.value.span.source_text {
            println!(
                "{}",
                format!("{}├── LitExpr: {}", "│   ".repeat(layer), literal)
            );
        }
    }
}

pub struct ExprUnary {
    pub operator: Token,
    pub expr: Box<Expr>,
}

impl DisplayTree for ExprUnary {
    fn display(&self, layer: usize) {
        let operator = &self.operator.span.source_text.as_ref().unwrap();

        println!("{}", format!("{}├── UnaryExpr", "│   ".repeat(layer)));
        println!("{}├── UnaryOp: {}", "│   ".repeat(layer + 1), operator);
        self.expr.display(layer + 1);
    }
}

pub struct ExprBinary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl DisplayTree for ExprBinary {
    fn display(&self, layer: usize) {
        let operator = &self.operator.span.source_text.as_ref().unwrap();
        println!("{}", format!("{}├── BinaryExpr", "│   ".repeat(layer)));
        self.left.display(layer + 1);
        println!("{}├── BinaryOp: {}", "│   ".repeat(layer + 1), operator);
        self.right.display(layer + 1);
    }
}

pub struct ExprParen {
    pub left_paren: Token,
    pub expr: Box<Expr>,
    pub right_paren: Token,
}

impl DisplayTree for ExprParen {
    fn display(&self, layer: usize) {
        self.expr.display(layer);
    }
}

pub enum Expr {
    Ident(ExprIdent),
    Literal(ExprLit),
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
            Self::Literal(lit) => lit.display(layer),
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
        let operator = input.next();
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
        let operator = input.next();
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
        let operator = input.next();
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
        let operator = input.next();
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
        let operator = input.next();
        let right = unary(input)?;
        Ok(Expr::Unary(ExprUnary {
            operator,
            expr: Box::new(right),
        }))
    } else {
        primary(input)
    }
}

fn primary(input: &mut ParseStream) -> Result<Expr, String> {
    match input.peek() {
        ident!() => Ok(Expr::Ident(input.parse()?)),
        token!(false) | token!(true) | token!(nil) | number!() => Ok(Expr::Literal(ExprLit {
            value: input.next(),
        })),
        token!('(') => Ok(Expr::Paren(ExprParen {
            left_paren: input.expect(token!('('))?,
            expr: input.parse()?,
            right_paren: input.expect(token!(')'))?,
        })),
        _ => {
            let start = input.cur().span.start;
            input.error_reporter().report("Expression expected", start);
            Err("Expression expected".to_owned())
        }
    }
}
