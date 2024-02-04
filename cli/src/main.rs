use std::env;
use std::fs;

use compiler::generator::Chunk;
use compiler::generator::Constant;
use compiler::generator::OpCode;
use compiler::scanner::lexer::Lexer;
use compiler::source_code::SourceCode;
use compiler::syntax::ast::Ast;
use compiler::syntax::display_tree::DisplayTree;
use compiler::syntax::parse::ParseStream;

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

    let mut chunk = Chunk::new();

    let c = chunk.add_constant(Constant::Value(1.2));

    chunk.write_op(OpCode::Constant);
    chunk.write_op_as_byte(c);
    chunk.write_op(OpCode::Return);

    println!("{:#?}", chunk);
    println!("{:#?}", chunk.disassemble());
}
