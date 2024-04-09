use std::time::Instant;

use clap::Parser;
use lumi_lxr::lexer::Lexer;
use lumi_lxr::source_code::SourceCode;
use lumi_psr::ast::Ast;
use lumi_psr::parser::ParseStream;
use lumi_vm::chunk::Chunk;
use lumi_vm::emitter::BytecodeEmitter;
use lumi_vm::vm::Vm;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    file: String,
    #[arg(short, long, default_value_t = true)]
    type_check: bool,
}

fn main() {
    let args = Args::parse();
    let core_source_code = SourceCode::from_file("core.ls").unwrap();
    let std_source_code = SourceCode::from_file("std.ls").unwrap();
    let source_code = SourceCode::from_file(&args.file).unwrap();

    let start_compilation_time = Instant::now();

    let mut chunk = Chunk::new();

    let result = vec![core_source_code, std_source_code, source_code]
        .iter()
        .all(|code| compile(code.clone(), &mut chunk).is_ok());

    if result {
        println!(
            "{}: {} milliseconds\n",
            "Compilation time",
            (Instant::now() - start_compilation_time).as_millis()
        );

        let mut vm = Vm::new(chunk);
        let start_execution_time = Instant::now();

        match vm.run() {
            Ok(_) => {}
            Err(runtime_error) => {
                eprintln!("{}", runtime_error);
            }
        }
        println!(
            "\n{}: {} milliseconds",
            "Execution time",
            (Instant::now() - start_execution_time).as_millis()
        );
    }
}

fn compile(source_code: SourceCode, chunk: &mut Chunk) -> Result<(), ()> {
    let mut lexer = Lexer::new(source_code);
    match lexer.tokens() {
        Ok(tokens) => {
            let mut parse_stream = ParseStream::new(tokens);
            match parse_stream.parse::<Ast>() {
                Ok(ast) => {
                    BytecodeEmitter::emit(&ast, chunk);
                    Ok(())
                }
                Err(error) => {
                    eprintln!("{}", error);
                    Err(())
                }
            }
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{}", error);
            }
            Err(())
        }
    }
}
