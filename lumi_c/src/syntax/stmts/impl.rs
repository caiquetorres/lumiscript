use crate::ident;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::r#type::Type;
use crate::syntax::span::Span;
use crate::syntax::symbols::brace::LeftBrace;
use crate::syntax::symbols::brace::RightBrace;
use crate::syntax::symbols::ident::Ident;
use crate::syntax::symbols::r#for::For;
use crate::token;

use crate::scanner::token::TokenKind;

use super::fun::StmtFun;

pub struct Impl {
    _span: Span,
}

impl Parse for Impl {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(Impl {
            _span: Span::from_token(input.expect(token!(impl))?),
        })
    }
}

pub struct Trait {
    ident: Ident,
    _for: For,
}

impl Parse for Trait {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(Trait {
            ident: input.parse()?,
            _for: input.parse()?,
        })
    }
}

impl DisplayTree for Trait {
    fn display(&self, layer: usize) {
        branch(&format!("Trait: {}", self.ident.name()), layer)
    }
}

impl Parse for Option<Trait> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        if input.peek() == ident!() && input.peek2() == token!(for) {
            Ok(Some(Trait {
                ident: input.parse()?,
                _for: input.parse()?,
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct StmtImpl {
    _impl: Impl,
    r#tr: Option<Trait>,
    ty: Type,
    _left_brace: LeftBrace,
    methods: Vec<StmtFun>,
    _right_brace: RightBrace,
}

impl Parse for StmtImpl {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(StmtImpl {
            _impl: input.parse()?,
            r#tr: input.parse()?,
            ty: input.parse()?,
            _left_brace: input.parse()?,
            methods: input.parse()?,
            _right_brace: input.parse()?,
        })
    }
}

impl DisplayTree for StmtImpl {
    fn display(&self, layer: usize) {
        branch("ImplStmt", layer);

        if let Some(tr) = &self.tr {
            tr.display(layer + 1);
        }

        self.ty.display(layer + 1);
        self.methods.display(layer + 1);
    }
}