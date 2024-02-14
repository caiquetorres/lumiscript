use clap::Parser;
use lumi_lxr::lexer::Lexer;
use lumi_lxr::utils::source_code::SourceCode;

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
            for token in tokens.iter() {
                println!("{:?}", token);
            }
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{}", error);
            }
        }
    }
}
