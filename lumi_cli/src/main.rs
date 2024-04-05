use std::time::Instant;

use clap::Parser;
use lumi_bc_e::emitter::BytecodeEmitter;
use lumi_lxr::lexer::Lexer;
use lumi_lxr::source_code::SourceCode;
use lumi_psr::ast::Ast;
use lumi_psr::parser::ParseStream;
use lumi_vm::VirtualMachine;

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
    let source_code = SourceCode::from_file(&args.file).unwrap();
    let start_compilation_time = Instant::now();
    let mut lexer = Lexer::new(source_code);
    match lexer.tokens() {
        Ok(tokens) => {
            let mut parse_stream = ParseStream::new(tokens);
            match parse_stream.parse::<Ast>() {
                Ok(ast) => {
                    // ast.display(0);
                    let chunk = BytecodeEmitter::emit(&ast);
                    println!(
                        "Compilation time: {} milliseconds",
                        (Instant::now() - start_compilation_time).as_millis()
                    );
                    println!("Chunk size: {:.2} kB\n", chunk.size());
                    let mut vm = VirtualMachine::new();
                    let start_execution_time = Instant::now();
                    println!("Output:");
                    if let Ok(_) = vm.run(chunk) {
                        println!(
                            "\nExecution time: {} milliseconds",
                            (Instant::now() - start_execution_time).as_millis()
                        );
                    }
                }
                Err(error) => eprintln!("{}", error),
            };
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{}", error);
            }
        }
    }
}
