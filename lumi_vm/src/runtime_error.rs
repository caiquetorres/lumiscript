use std::fmt::Display;

use colored::Colorize;
use lumi_lxr::span::Span;

use crate::frame::Trace;

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
    span: Span,
    stack_trace: Vec<Trace>,
}

impl RuntimeError {
    pub fn new(message: &str, span: Span, stack_trace: Vec<Trace>) -> Self {
        Self {
            message: message.to_owned(),
            span,
            stack_trace,
        }
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn stack_trace(&self) -> &Vec<Trace> {
        &self.stack_trace
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("{}: {}", "runtime error".red().bold(), self.message);
        let line = self.span.start().line();
        let column = self.span.start().column();
        let line_content = self
            .span
            .source_code()
            .code()
            .lines()
            .nth(line - 1)
            .unwrap();
        output = output
            + &format!(
                "\n{} {}:{}:{}\n",
                "-->".blue().bold(),
                self.span.source_code().file_path(),
                line,
                column
            )
            + &format!("      {}\n", "|".blue().bold())
            + &format!(
                "{: >5} {} {}\n",
                line.to_string().blue().bold(),
                "|".blue().bold(),
                line_content
            )
            + &format!("      {}", "|".blue().bold())
            + &format!(
                "{}{}",
                " ".repeat(column),
                "^".repeat(self.span.end().column() - self.span.start().column())
                    .red()
                    .bold(),
            );

        let mut prev_span = self.span.clone();
        println!("{:#?}", self.stack_trace);
        for trace in self.stack_trace.iter().rev() {
            let line = prev_span.start().line();
            let column = prev_span.start().column();
            let function_and_file = format!(
                "{}:{}:{}",
                prev_span.source_code().file_path(),
                line,
                column
            );
            if let Some(class_name) = trace.class_name.clone() {
                output = output
                    + &format!(
                        "\n{}{} {}.{} {}",
                        " ".repeat(4),
                        "at".black(),
                        class_name,
                        trace.function_name.underline(),
                        function_and_file.black()
                    );
            } else {
                output = output
                    + &format!(
                        "\n{}{} {} {}",
                        " ".repeat(4),
                        "at".black(),
                        trace.function_name.underline(),
                        function_and_file.black()
                    );
            }
            prev_span = trace.span.clone();
        }
        let function_and_file = format!(
            "{}:{}:{}",
            prev_span.source_code().file_path(),
            line,
            column
        );
        output = output
            + &format!(
                "\n{}{} {}",
                " ".repeat(4),
                "at".black(),
                function_and_file.black()
            );

        write!(f, "{}", output)
    }
}
