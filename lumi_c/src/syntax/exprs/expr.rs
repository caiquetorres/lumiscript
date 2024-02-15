use lumi_lxr::span::Span;
use lumi_lxr::token::TokenKind;

use crate::compile_error::CompileError;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::symbols::LeftParen;
use crate::syntax::symbols::RightParen;

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
    pub fn parse_without_eager_brace(input: &mut ParseStream) -> Result<Self, CompileError> {
        ambiguous_expr(input, false)
    }
}

impl Parse for Expr {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        ambiguous_expr(input, true)
    }
}

impl Parse for Box<Expr> {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        ambiguous_expr(input, true).map(Box::new)
    }
}

impl Parse for Vec<Expr> {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        let mut exprs = vec![];

        if input.peek() == TokenKind::RightParen {
            Ok(exprs)
        } else {
            exprs.push(input.parse()?);

            while input.peek() == TokenKind::Comma {
                input.expect(TokenKind::Comma)?;
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

fn ambiguous_expr(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, CompileError> {
    assignment(input, allow_struct)
}

fn assignment(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, CompileError> {
    let mut expr = equality(input, allow_struct)?;

    if input.peek() == TokenKind::Equal {
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

fn equality(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, CompileError> {
    let mut left = comparison(input, allow_struct)?;

    while input.peek() == TokenKind::BangEqual || input.peek() == TokenKind::EqualEqual {
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

fn comparison(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, CompileError> {
    let mut left: Expr = range(input, allow_struct)?;

    while input.peek() == TokenKind::Greater
        || input.peek() == TokenKind::GreaterEqual
        || input.peek() == TokenKind::Less
        || input.peek() == TokenKind::LessEqual
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

fn range(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, CompileError> {
    let mut left: Expr = term(input, allow_struct)?;

    while input.peek() == TokenKind::DotDot || input.peek() == TokenKind::DotDotEqual {
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

fn term(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, CompileError> {
    let mut left = factor(input, allow_struct)?;

    while input.peek() == TokenKind::Plus || input.peek() == TokenKind::Minus {
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

fn factor(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, CompileError> {
    let mut left = unary(input, allow_struct)?;

    while input.peek() == TokenKind::Star || input.peek() == TokenKind::Slash {
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

fn unary(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, CompileError> {
    if input.peek() == TokenKind::Plus
        || input.peek() == TokenKind::Minus
        || input.peek() == TokenKind::Bang
    {
        Ok(Expr::Unary(ExprUnary {
            operator: input.parse()?,
            expr: Box::new(unary(input, allow_struct)?),
        }))
    } else {
        call(input, allow_struct)
    }
}

fn call(input: &mut ParseStream, allow_struct: bool) -> Result<Expr, CompileError> {
    let mut expr = primary(input)?;

    if input.peek() == TokenKind::RightBrace && allow_struct {
        Ok(Expr::Class(ExprClass {
            class: Box::new(expr),
            left_brace: input.parse()?,
            field_inits: input.parse()?,
            right_brace: input.parse()?,
        }))
    } else {
        while input.peek() == TokenKind::Dot || input.peek() == TokenKind::LeftParen {
            expr = match input.peek() {
                TokenKind::LeftParen => Expr::Call(ExprCall {
                    callee: Box::new(expr),
                    left_paren: input.parse()?,
                    args: input.parse()?,
                    right_paren: input.parse()?,
                }),
                TokenKind::Dot => Expr::Get(ExprGet {
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

fn primary(input: &mut ParseStream) -> Result<Expr, CompileError> {
    match input.peek() {
        TokenKind::Ident => Ok(Expr::Ident(input.parse()?)),
        TokenKind::Nil => Ok(Expr::Lit(ExprLit::Nil {
            span: Span::from(input.next().span()),
        })),
        TokenKind::Number => Ok(Expr::Lit(ExprLit::Num {
            span: Span::from(input.next().span()),
        })),
        TokenKind::True | TokenKind::False => Ok(Expr::Lit(ExprLit::Bool {
            span: Span::from(input.next().span()),
        })),
        TokenKind::LeftParen => {
            input.parse::<LeftParen>()?;
            let expr: Expr = input.parse()?;
            input.parse::<RightParen>()?;
            Ok(expr)
        }
        _ => Err(CompileError::new(
            "Expression expected",
            input.cur().span().start(),
            input.source_code(),
        )),
    }
}
