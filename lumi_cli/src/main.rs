use std::fs;

use clap::Parser;
use compiler::compiler::Compiler;
use virtual_machine::vm::Vm;

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

    let core_lib = fs::read_to_string("lumi_core/libs/core.ls").unwrap();
    let code = fs::read_to_string(&args.file).unwrap();

    match Compiler::compile(&[core_lib, code]) {
        Ok(chunk) => {
            let mut vm = Vm::new();
            if let Err(runtime_error) = vm.run(chunk) {
                println!("{:?}", runtime_error);
            }

            Ok(())
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{}", error);
            }
            Err("Compile error".to_owned())
        }
    }
}
