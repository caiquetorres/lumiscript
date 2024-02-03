pub mod error_reporter;
pub mod line_column;
pub mod scanner;
pub mod source_code;
pub mod syntax;

use std::env;
use std::fs;

use scanner::lexer::Lexer;
use scanner::token::TokenKind;
use source_code::SourceCode;
use syntax::display_tree::DisplayTree;
use syntax::parse::ParseStream;
use syntax::stmts::stmt::Stmt;

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

    // for token in &tokens {
    //     println!("{:?}", token);
    // }

    if lexer.error_reporter().has_errors() {
        lexer.error_reporter().show();
    } else {
        let mut parser = ParseStream::new(tokens, source_code.clone());
        let mut stmts = vec![];

        while parser.peek() != token!(eof) {
            if let Ok(stmt) = parser.parse::<Stmt>() {
                stmts.push(stmt);
            } else {
                break;
            }
        }

        if parser.error_reporter().has_errors() {
            parser.error_reporter().show();
        } else {
            stmts.display(0);
        }
    }
}
