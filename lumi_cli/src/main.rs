use clap::Parser;
use lumi_bc_e::emitter::BytecodeEmitter;
use lumi_lxr::lexer::Lexer;
use lumi_lxr::source_code::SourceCode;
use lumi_psr::ast::Ast;
use lumi_psr::display_tree::DisplayTree;
use lumi_psr::parser::ParseStream;
use virtual_machine::VirtualMachine;

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
    let mut lexer = Lexer::new(source_code);
    match lexer.tokens() {
        Ok(tokens) => {
            let mut parse_stream = ParseStream::new(tokens);
            match parse_stream.parse::<Ast>() {
                Ok(ast) => {
                    ast.display(0);
                    let chunk = BytecodeEmitter::emit(&ast);
                    println!("");
                    println!("{:?}\n", chunk);
                    let mut vm = VirtualMachine::new();
                    if let Ok(_) = vm.run(chunk) {}
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
