use std::fs;

use clap::Parser;
use compiler::generator::generator::Generator;
use compiler::generator::obj::ObjFunc;
use compiler::scanner::lexer::Lexer;
use compiler::syntax::ast::Ast;
use compiler::syntax::parse::ParseStream;
use lumi_core::source_code::SourceCode;
use type_checker::TypeChecker;
use virtual_machine::VirtualMachine;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long, default_value_t = true)]
    type_check: bool,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    if let Ok(content) = fs::read_to_string(&args.file) {
        run(&content, args.type_check);
        Ok(())
    } else {
        Err("File not found".to_owned())
    }
}

fn run(source_code: &str, should_type_check: bool) {
    let source_code = SourceCode::new(source_code);

    let mut lexer = Lexer::new(source_code.clone());
    let tokens = lexer.lex();

    if lexer.error_reporter().has_errors() {
        lexer.error_reporter().show();
        return;
    }

    let mut parser = ParseStream::new(tokens, source_code.clone());
    let ast: Result<Ast, String> = parser.parse();

    if parser.error_reporter().has_errors() {
        parser.error_reporter().show();
        return;
    }

    let ast = ast.unwrap();
    // ast.display(0);

    if should_type_check {
        TypeChecker::check(&ast);
    }

    let chunk = Generator::generate(&ast);
    VirtualMachine::run(ObjFunc::root(chunk));
}
