use lumi_lxr::span::Span;
use lumi_lxr::token::TokenKind;

use crate::display_tree::DisplayTree;
use crate::parse::Parse;
use crate::parser::ParseError;
use crate::parser::ParseStream;

use self::binary::BinaryExpr;
use self::call::CallExpr;
use self::class::ClassExpr;
use self::get::GetExpr;
use self::ident::IdentExpr;
use self::lit::LitExpr;
use self::paren::ParenExpr;
use self::unary::UnaryExpr;

pub mod binary;
pub mod call;
pub mod class;
pub mod get;
pub mod ident;
pub mod lit;
pub mod paren;
pub mod unary;

#[derive(Debug)]
pub enum Expr {
    Ident(IdentExpr),
    Lit(LitExpr),
    Paren(ParenExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Class(ClassExpr),
    Call(CallExpr),
    Get(GetExpr),
}

impl Expr {
    pub fn span(&self) -> &Span {
        match self {
            Self::Ident(ident) => ident.span(),
            Self::Lit(lit) => lit.span(),
            Self::Paren(paren) => paren.span(),
            Self::Unary(unary) => unary.span(),
            Self::Binary(binary) => binary.span(),
            Self::Class(cls) => cls.span(),
            Self::Call(call) => call.span(),
            Self::Get(get) => get.span(),
        }
    }

    pub fn parse_without_eager_brace(input: &mut ParseStream) -> Result<Self, ParseError> {
        ambiguous_expr(input, false)
    }
}

impl Parse for Expr {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        ambiguous_expr(input, true)
    }
}

impl DisplayTree for Expr {
    fn display(&self, layer: usize) {
        match self {
            Self::Ident(ident) => ident.display(layer),
            Self::Lit(lit) => lit.display(layer),
            Self::Paren(paren) => paren.display(layer),
            Self::Unary(unary) => unary.display(layer),
            Self::Binary(binary) => binary.display(layer),
            Self::Class(cls) => cls.display(layer),
            Self::Call(call) => call.display(layer),
            Self::Get(get) => get.display(layer),
        }
    }
}

fn ambiguous_expr(input: &mut ParseStream, allow_class: bool) -> Result<Expr, ParseError> {
    assignment(input, allow_class)
}

fn assignment(input: &mut ParseStream, allow_class: bool) -> Result<Expr, ParseError> {
    let mut left = equality(input, allow_class)?;
    if input.peek().kind() == TokenKind::Equal {
        let operator = input.parse()?;
        let right = assignment(input, allow_class)?;
        left = Expr::Binary(BinaryExpr::new(left, operator, right));
    }
    Ok(left)
}

fn equality(input: &mut ParseStream, allow_class: bool) -> Result<Expr, ParseError> {
    let mut left = comparison(input, allow_class)?;
    while matches!(
        input.peek().kind(),
        TokenKind::BangEqual | TokenKind::EqualEqual
    ) {
        let operator = input.parse()?;
        let right = comparison(input, allow_class)?;
        left = Expr::Binary(BinaryExpr::new(left, operator, right));
    }
    Ok(left)
}

fn comparison(input: &mut ParseStream, allow_class: bool) -> Result<Expr, ParseError> {
    let mut left: Expr = range(input, allow_class)?;
    while matches!(
        input.peek().kind(),
        TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual
    ) {
        let operator = input.parse()?;
        let right = range(input, allow_class)?;
        left = Expr::Binary(BinaryExpr::new(left, operator, right));
    }
    Ok(left)
}

fn range(input: &mut ParseStream, allow_class: bool) -> Result<Expr, ParseError> {
    let mut left: Expr = term(input, allow_class)?;
    while input.peek().kind() == TokenKind::DotDot || input.peek().kind() == TokenKind::DotDotEqual
    {
        let operator = input.parse()?;
        let right = term(input, allow_class)?;
        left = Expr::Binary(BinaryExpr::new(left, operator, right));
    }
    Ok(left)
}

fn term(input: &mut ParseStream, allow_class: bool) -> Result<Expr, ParseError> {
    let mut left = factor(input, allow_class)?;
    while matches!(input.peek().kind(), TokenKind::Plus | TokenKind::Minus) {
        let operator = input.parse()?;
        let right = factor(input, allow_class)?;
        left = Expr::Binary(BinaryExpr::new(left, operator, right));
    }
    Ok(left)
}

fn factor(input: &mut ParseStream, allow_class: bool) -> Result<Expr, ParseError> {
    let mut left = unary(input, allow_class)?;
    while matches!(input.peek().kind(), TokenKind::Star | TokenKind::Slash) {
        let operator = input.parse()?;
        let right = unary(input, allow_class)?;
        left = Expr::Binary(BinaryExpr::new(left, operator, right));
    }
    Ok(left)
}

fn unary(input: &mut ParseStream, allow_class: bool) -> Result<Expr, ParseError> {
    if matches!(
        input.peek().kind(),
        TokenKind::Plus | TokenKind::Minus | TokenKind::Bang
    ) {
        Ok(Expr::Unary(UnaryExpr::new(
            input.parse()?,
            unary(input, allow_class)?,
        )))
    } else {
        call(input, allow_class)
    }
}

fn call(input: &mut ParseStream, allow_class: bool) -> Result<Expr, ParseError> {
    let mut expr = primary(input)?;
    while matches!(
        input.peek().kind(),
        TokenKind::Dot | TokenKind::LeftParen | TokenKind::LeftBrace
    ) {
        expr = match input.peek().kind() {
            TokenKind::LeftParen => Expr::Call(CallExpr::new(
                expr,
                input.parse()?,
                input.parse()?,
                input.parse()?,
            )),
            TokenKind::Dot => Expr::Get(GetExpr::new(expr, input.parse()?, input.parse()?)),
            _ => {
                if !allow_class {
                    break;
                }
                Expr::Class(ClassExpr::new(
                    expr,
                    input.parse()?,
                    input.parse()?,
                    input.parse()?,
                ))
            }
        }
    }
    Ok(expr)
}

fn primary(input: &mut ParseStream) -> Result<Expr, ParseError> {
    match input.peek().kind() {
        TokenKind::Ident => Ok(Expr::Ident(IdentExpr::new(input.parse()?))),
        TokenKind::Number => Ok(Expr::Lit(LitExpr::num(input.next().span()))),
        TokenKind::Nil => Ok(Expr::Lit(LitExpr::nil(input.next().span()))),
        TokenKind::True | TokenKind::False => Ok(Expr::Lit(LitExpr::bool(input.next().span()))),
        TokenKind::LeftParen => Ok(Expr::Paren(ParenExpr::new(
            input.parse()?,
            input.parse()?,
            input.parse()?,
        ))),
        _ => Err(ParseError {
            message: "Expression expected".to_owned(),
            span: input.peek().span().clone(),
            source_code: input.peek().span().source_code().clone(),
        }),
    }
}
