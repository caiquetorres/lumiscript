use lumi_lxr::token::TokenKind;

use crate::compile_error::CompileError;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::r#type::Type;
use crate::syntax::symbols::For;
use crate::syntax::symbols::Ident;
use crate::syntax::symbols::LeftBrace;
use crate::syntax::symbols::RightBrace;
use crate::syntax_symbol;

use super::fun::StmtFun;

syntax_symbol!(Impl, TokenKind::Impl);

pub struct Trait {
    ident: Ident,
    _for: For,
}

impl Trait {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }
}

impl Parse for Trait {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Trait {
            ident: input.parse()?,
            _for: input.parse()?,
        })
    }
}

impl DisplayTree for Trait {
    fn display(&self, layer: usize) {
        branch(&format!("Trait: {}", self.ident.source_text()), layer)
    }
}

impl Parse for Option<Trait> {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        if input.peek() == TokenKind::Ident && input.peek2() == TokenKind::For {
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
    tr: Option<Trait>,
    ty: Type,
    _left_brace: LeftBrace,
    methods: Vec<StmtFun>,
    _right_brace: RightBrace,
}

impl StmtImpl {
    pub fn tr(&self) -> Option<&Trait> {
        self.tr.as_ref()
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn methods(&self) -> &Vec<StmtFun> {
        &self.methods
    }
}

impl Parse for StmtImpl {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(StmtImpl {
            _impl: input.parse()?,
            tr: input.parse()?,
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
