pub mod error_reporter;
pub mod line_column;
pub mod scanner;
pub mod source_code;
pub mod syntax;

use std::env;
use std::fs;

use scanner::lexer::Lexer;
use source_code::SourceCode;
use syntax::ast::Ast;
use syntax::display_tree::DisplayTree;
use syntax::parse::ParseStream;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args[1].clone();
    let content = fs::read_to_string(file_path).unwrap();

    compile(&content);
}

fn compile(source_code: &str) {
    let source_code = SourceCode::new(source_code);

    let mut lexer = Lexer::new(source_code.clone());
    let tokens = lexer.lex();

    if lexer.error_reporter().has_errors() {
        lexer.error_reporter().show();
    } else {
        let mut parser = ParseStream::new(tokens, source_code.clone());

        if let Ok(ast) = parser.parse::<Ast>() {
            ast.display(0);
        } else {
            parser.error_reporter().show();
        }
    }
}
