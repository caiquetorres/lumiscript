use crate::line_column::LineColumn;
use crate::source_code::SourceCode;

#[derive(Debug)]
struct ErrorHandler {
    message: String,
    line_column: LineColumn,
}

#[derive(Debug)]
pub struct ErrorReporter {
    source_code: SourceCode,
    errors: Vec<ErrorHandler>,
}

impl ErrorReporter {
    pub fn new(source_code: SourceCode) -> Self {
        Self {
            source_code,
            errors: vec![],
        }
    }

    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn report(&mut self, message: &str, line_column: LineColumn) {
        self.errors.push(ErrorHandler {
            message: message.to_owned(),
            line_column,
        });
    }

    pub fn show(&self) {
        for error in self.errors.iter() {
            let line = error.line_column.line;
            let column = error.line_column.column;

            let line_content = self.source_code.code().lines().nth(line - 1).unwrap();

            let output = format!(
                "Error: {} at Line {} at Column {}\n\n",
                error.message, line, column,
            ) + &format!("    {} | {}\n", line, line_content)
                + &format!("{}^-- Here.", " ".repeat(column + 7));

            eprintln!("{}\n", output);
        }
    }
}
