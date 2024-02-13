use std::fmt::Display;

use lumi_core::line_column::LineColumn;
use lumi_core::source_code::SourceCode;

#[derive(Debug, Clone)]
pub struct CompileError {
    message: String,
    line_column: LineColumn,
    source_code: SourceCode,
}

impl CompileError {
    pub fn new(message: &str, line_column: LineColumn, source_code: SourceCode) -> Self {
        Self {
            message: message.to_owned(),
            line_column,
            source_code,
        }
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn line_column(&self) -> LineColumn {
        self.line_column
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self.line_column.line;
        let column = self.line_column.column;

        let line_content = self.source_code.code().lines().nth(line - 1).unwrap();

        let output = format!(
            "Compile Error: {} at Line {} at Column {}\n\n",
            self.message, line, column,
        ) + &format!("    {} | {}\n", line, line_content)
            + &format!("{}^-- Here.", " ".repeat(column + 7));

        writeln!(f, "{}\n", output)
    }
}
