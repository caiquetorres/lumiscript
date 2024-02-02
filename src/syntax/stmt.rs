use crate::ident;
use crate::scanner::token::Token;
use crate::scanner::token::TokenKind;
use crate::token;

use super::expr::DisplayTree;
use super::expr::Expr;
use super::parse::Parse;
use super::parse::ParseStream;

pub struct StmtLet {
    _let: Token,
    ident: Token,
    ty: Option<Type>,
    _equal: Token,
    expr: Expr,
    _semicolon: Token,
}

impl DisplayTree for StmtLet {
    fn display(&self, layer: usize) {
        println!("{}", format!("{}├── LetStmt", "│   ".repeat(layer)));
        println!(
            "{}├── Ident: {}",
            "│   ".repeat(layer + 1),
            self.ident.span.source_text.as_ref().unwrap()
        );
        if let Some(ty) = &self.ty {
            println!(
                "{}├── Type: {}{}",
                "│   ".repeat(layer + 1),
                ty.ident.span.source_text.as_ref().unwrap(),
                if ty.nullable { "?" } else { "" }
            );
        }
        self.expr.display(layer + 1);
    }
}

impl Parse for StmtLet {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(StmtLet {
            _let: input.expect(token!(let))?,
            ident: input.expect(ident!())?,
            ty: input.parse()?,
            _equal: input.expect(token!(=))?,
            expr: input.parse()?,
            _semicolon: input.expect(token!(;))?,
        })
    }
}

pub struct StmtPrint {
    _println: Token,
    expr: Expr,
    _semicolon: Token,
}

impl DisplayTree for StmtPrint {
    fn display(&self, layer: usize) {
        println!("{}", format!("{}├── PrintlnStmt", "│   ".repeat(layer)));
        self.expr.display(layer + 1);
    }
}

impl Parse for StmtPrint {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(StmtPrint {
            _println: input.expect(token!(println))?,
            expr: input.parse()?,
            _semicolon: input.expect(token!(;))?,
        })
    }
}

pub struct StmtBlock {
    _left_brace: Token,
    stmts: Vec<Box<Stmt>>,
    _right_brace: Token,
}

impl Parse for StmtBlock {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(StmtBlock {
            _left_brace: input.expect(token!('{'))?,
            stmts: {
                let mut stmts = vec![];

                while input.peek() != token!('}') && input.peek() != token!(eof) {
                    stmts.push(Box::new(input.parse::<Stmt>()?));
                }

                stmts
            },
            _right_brace: input.expect(token!('}'))?,
        })
    }
}

impl DisplayTree for StmtBlock {
    fn display(&self, layer: usize) {
        println!("{}", format!("{}├── BlockStmt", "│   ".repeat(layer)));
        for stmt in &self.stmts {
            stmt.display(layer + 1);
        }
    }
}

pub struct StmtExpr {
    expr: Expr,
    _semicolon: Token,
}

impl Parse for StmtExpr {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(StmtExpr {
            expr: input.parse()?,
            _semicolon: input.expect(token!(;))?,
        })
    }
}

impl DisplayTree for StmtExpr {
    fn display(&self, layer: usize) {
        self.expr.display(layer);
    }
}

pub struct Type {
    _colon: Token,
    ident: Token,
    nullable: bool,
}

impl Parse for Option<Type> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        if input.peek() == token!(:) {
            Ok(Some(input.parse()?))
        } else {
            Ok(None)
        }
    }
}

impl Parse for Type {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(Type {
            _colon: input.expect(token!(:))?,
            ident: input.expect(ident!())?,
            nullable: {
                if input.peek() == token!(?) {
                    input.next();
                    true
                } else {
                    false
                }
            },
        })
    }
}

pub struct ReturnType {
    _arrow: Token,
    ident: Token,
    nullable: bool,
}

impl Parse for ReturnType {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(ReturnType {
            _arrow: input.expect(token!(->))?,
            ident: input.expect(ident!())?,
            nullable: {
                if input.peek() == token!(?) {
                    input.next();
                    true
                } else {
                    false
                }
            },
        })
    }
}

impl Parse for Option<ReturnType> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        if input.peek() == token!(->) {
            Ok(Some(input.parse()?))
        } else {
            Ok(None)
        }
    }
}

pub struct Param {
    ident: Token,
    ty: Type,
}

impl Parse for Param {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(Param {
            ident: input.expect(ident!())?,
            ty: input.parse()?,
        })
    }
}

impl DisplayTree for Param {
    fn display(&self, layer: usize) {
        println!("{}", format!("{}├── Param", "│   ".repeat(layer)));
        println!(
            "{}├── Ident: {}",
            "│   ".repeat(layer + 1),
            self.ident.span.source_text.as_ref().unwrap()
        );
        println!(
            "{}├── Type: {}{}",
            "│   ".repeat(layer + 1),
            self.ty.ident.span.source_text.as_ref().unwrap(),
            if self.ty.nullable { "?" } else { "" }
        );
    }
}

impl Parse for Vec<Param> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        let mut params = vec![];

        if input.peek() == token!(')') {
            Ok(params)
        } else {
            params.push(input.parse()?);

            while input.peek() == token!(,) {
                input.expect(token!(,))?;
                params.push(input.parse()?);
            }

            Ok(params)
        }
    }
}

impl DisplayTree for Vec<Param> {
    fn display(&self, layer: usize) {
        if !self.is_empty() {
            println!("{}", format!("{}├── Params", "│   ".repeat(layer)));
            for param in self {
                param.display(layer + 1);
            }
        }
    }
}

pub struct StmtFun {
    _fun: Token,
    ident: Token,
    _left_paren: Token,
    params: Vec<Param>,
    _right_paren: Token,
    return_type: Option<ReturnType>,
    _left_brace: Token,
    body: Vec<Box<Stmt>>,
    _right_brace: Token,
}

impl Parse for StmtFun {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(StmtFun {
            _fun: input.expect(token!(fun))?,
            ident: input.expect(ident!())?,
            _left_paren: input.expect(token!('('))?,
            params: input.parse()?,
            _right_paren: input.expect(token!(')'))?,
            return_type: input.parse()?,
            _left_brace: input.expect(token!('{'))?,
            body: {
                let mut stmts = vec![];

                while input.peek() != token!('}') && input.peek() != token!(eof) {
                    stmts.push(Box::new(input.parse::<Stmt>()?));
                }

                stmts
            },
            _right_brace: input.expect(token!('}'))?,
        })
    }
}

impl DisplayTree for StmtFun {
    fn display(&self, layer: usize) {
        println!("{}", format!("{}├── FunStmt", "│   ".repeat(layer)));
        println!(
            "{}├── Ident: {}",
            "│   ".repeat(layer + 1),
            self.ident.span.source_text.as_ref().unwrap(),
        );

        self.params.display(layer + 1);

        if let Some(return_type) = &self.return_type {
            println!(
                "{}├── Return: {}{}",
                "│   ".repeat(layer + 1),
                return_type.ident.span.source_text.as_ref().unwrap(),
                if return_type.nullable { "?" } else { "" }
            );
        }

        if !self.body.is_empty() {
            println!("{}├── Body", "│   ".repeat(layer + 1),);
            for stmt in &self.body {
                stmt.display(layer + 2);
            }
        }
    }
}

pub enum Stmt {
    Let(StmtLet),
    Print(StmtPrint),
    Block(StmtBlock),
    Fun(StmtFun),
    Expr(StmtExpr),
}

impl DisplayTree for Stmt {
    fn display(&self, layer: usize) {
        match self {
            Self::Let(r#let) => r#let.display(layer),
            Self::Print(print) => print.display(layer),
            Self::Block(block) => block.display(layer),
            Self::Expr(expr) => expr.display(layer),
            Self::Fun(fun) => fun.display(layer),
        }
    }
}

impl Parse for Stmt {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        ambiguous_stmt(input)
    }
}

impl Parse for Vec<Stmt> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        let mut stmts = vec![];

        while input.peek() != token!(eof) {
            stmts.push(input.parse::<Stmt>()?);
        }

        Ok(stmts)
    }
}

impl Parse for Vec<Box<Stmt>> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        let mut stmts = vec![];

        while input.peek() != token!(eof) {
            stmts.push(Box::new(input.parse::<Stmt>()?));
        }

        Ok(stmts)
    }
}

fn ambiguous_stmt(input: &mut ParseStream) -> Result<Stmt, String> {
    match input.peek() {
        token!('{') => Ok(Stmt::Block(input.parse::<StmtBlock>()?)),
        token!(let) => Ok(Stmt::Let(input.parse::<StmtLet>()?)),
        token!(println) => Ok(Stmt::Print(input.parse::<StmtPrint>()?)),
        token!(fun) => Ok(Stmt::Fun(input.parse::<StmtFun>()?)),
        _ => Ok(Stmt::Expr(input.parse::<StmtExpr>()?)),
    }
}
