use crate::ident;
use crate::number;

use crate::scanner::token::token;
use crate::scanner::token::TokenKind;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;

use super::assign::ExprAssign;
use super::binary::ExprBinary;
use super::call::ExprCall;
use super::class::ExprClass;
use super::get::ExprGet;
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
    Get(ExprGet),
    Call(ExprCall),
    Assign(ExprAssign),
    Class(ExprClass),
}

impl Expr {
    pub fn parse_without_eager_brace(input: &mut ParseStream) -> Result<Self, String> {
        ambiguous_expr(input, false)
    }
}

impl Parse for Expr {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        ambiguous_expr(input, true)
    }
}

impl Parse for Box<Expr> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        ambiguous_expr(input, true).map(Box::new)
    }
}

impl Parse for Vec<Expr> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        let mut exprs = vec![];

        if input.peek() == token!(')') {
            Ok(exprs)
        } else {
            exprs.push(input.parse()?);

            while input.peek() == token!(,) {
                input.expect(token!(,))?;
                exprs.push(input.parse()?);
            }

            Ok(exprs)
        }
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
            Self::Get(get) => get.display(layer),
            Self::Call(call) => call.display(layer),
            Self::Assign(assign) => assign.display(layer),
            Self::Class(class) => class.display(layer),
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

fn ambiguous_expr(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, String> {
    assignment(input, allow_struct)
}

fn assignment(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, String> {
    let mut expr = equality(input, allow_struct)?;

    if input.peek() == token!(=) {
        let operator = input.parse()?;
        let right = assignment(input, allow_struct)?;

        expr = Expr::Assign(ExprAssign {
            left: Box::new(expr),
            operator,
            right: Box::new(right),
        })
    }

    Ok(expr)
}

fn equality(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, String> {
    let mut left = comparison(input, allow_struct)?;

    while input.peek() == token!(!=) || input.peek() == token!(==) {
        let operator = input.parse()?;
        let right = comparison(input, allow_struct)?;

        left = Expr::Binary(ExprBinary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        });
    }

    Ok(left)
}

fn comparison(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, String> {
    let mut left: Expr = range(input, allow_struct)?;

    while input.peek() == token!(>)
        || input.peek() == token!(>=)
        || input.peek() == token!(<)
        || input.peek() == token!(<=)
    {
        let operator = input.parse()?;
        let right = range(input, allow_struct)?;

        left = Expr::Binary(ExprBinary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        });
    }

    Ok(left)
}

fn range(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, String> {
    let mut left: Expr = term(input, allow_struct)?;

    while input.peek() == token!(..) || input.peek() == token!(..=) {
        let operator = input.parse()?;
        let right = term(input, allow_struct)?;

        left = Expr::Binary(ExprBinary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        });
    }

    Ok(left)
}

fn term(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, String> {
    let mut left = factor(input, allow_struct)?;

    while input.peek() == token!(+) || input.peek() == token!(-) {
        let operator = input.parse()?;
        let right = factor(input, allow_struct)?;

        left = Expr::Binary(ExprBinary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        });
    }

    Ok(left)
}

fn factor(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, String> {
    let mut left = unary(input, allow_struct)?;

    while input.peek() == token!(*) || input.peek() == token!(/) {
        let operator = input.parse()?;
        let right = unary(input, allow_struct)?;

        left = Expr::Binary(ExprBinary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        });
    }

    Ok(left)
}

fn unary(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, String> {
    if input.peek() == token!(+) || input.peek() == token!(-) || input.peek() == token!(!) {
        Ok(Expr::Unary(ExprUnary {
            operator: input.parse()?,
            expr: Box::new(unary(input, allow_struct)?),
        }))
    } else {
        call(input, allow_struct)
    }
}

fn call(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, String> {
    let mut expr = primary(input)?;

    if input.peek() == token!('{') && allow_struct {
        Ok(Expr::Class(ExprClass {
            class: Box::new(expr),
            left_brace: input.parse()?,
            field_inits: input.parse()?,
            right_brace: input.parse()?,
        }))
    } else {
        while input.peek() == token!(.) || input.peek() == token!('(') {
            expr = match input.peek() {
                token!('(') => Expr::Call(ExprCall {
                    callee: Box::new(expr),
                    left_paren: input.parse()?,
                    args: input.parse()?,
                    right_paren: input.parse()?,
                }),
                token!(.) => Expr::Get(ExprGet {
                    expr: Box::new(expr),
                    dot: input.parse()?,
                    ident: input.parse()?,
                }),
                _ => unreachable!(),
            }
        }

        Ok(expr)
    }
}

fn primary(input: &mut ParseStream) -> Result<Expr, String> {
    match input.peek() {
        ident!() => Ok(Expr::Ident(input.parse()?)),
        token!(nil) => Ok(Expr::Lit(ExprLit::Nil {
            span: Span::from_token(input.next()),
        })),
        number!() => Ok(Expr::Lit(ExprLit::Num {
            span: Span::from_token(input.next()),
        })),
        token!(false) | token!(true) => Ok(Expr::Lit(ExprLit::Bool {
            span: Span::from_token(input.next()),
        })),
        token!('(') => Ok(Expr::Paren(ExprParen {
            left_paren: input.parse()?,
            expr: input.parse()?,
            right_paren: input.parse()?,
        })),
        _ => {
            let start = input.cur().span.start;
            input.error_reporter().report("Expression expected", start);
            Err("Expression expected".to_owned())
        }
    }
}
