use lumi_core::source_code::SourceCode;
use lumi_lxr::lexer::Lexer;

use crate::{
    compile_error::CompileError,
    generator::{chunk::Chunk, generator::Generator},
    syntax::{ast::Ast, parse::ParseStream},
};

pub struct Compiler;

impl Compiler {
    pub fn compile(source_codes: &[String]) -> Result<Chunk, Vec<CompileError>> {
        // let mut root_ast: Option<Ast> = None;

        // for source_code in source_codes {
        //     let source_code = SourceCode::new(&source_code);

        //     let mut lexer = Lexer::new(source_code.clone());
        //     let tokens = lexer.tokens()?;

        //     let mut parser = ParseStream::new(tokens, source_code.clone());
        //     let ast = parser.parse::<Ast>().map_err(|err| vec![err])?;

        //     if let Some(root_ast) = root_ast.as_mut() {
        //         root_ast.merge(ast)
        //     } else {
        //         root_ast = Some(ast);
        //     }
        // }

        // Ok(Generator::generate(&root_ast.unwrap()))

        todo!()
    }
}
