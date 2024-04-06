use std::fmt::Display;

use colored::Colorize;
use lumi_lxr::span::Span;

use crate::frame::Trace;

type StackTrace = Vec<Trace>;

pub enum RuntimeError {
    SymbolNotFound {
        symbol_name: String,
        span: Span,
        stack_trace: StackTrace,
    },
    CannotReadProperty {
        class_name: String,
        property_name: String,
        span: Span,
        stack_trace: StackTrace,
    },
    InvalidBinaryOperands {
        type_name_left: String,
        type_name_right: String,
        span: Span,
        stack_trace: StackTrace,
    },
    SymbolNotCallable {
        symbol_name: String,
        span: Span,
        stack_trace: StackTrace,
    },
    InvalidInstantiation {
        symbol_name: String,
        span: Span,
        stack_trace: StackTrace,
    },
}

impl RuntimeError {
    fn message(&self) -> String {
        match self {
            Self::SymbolNotFound { symbol_name, .. } => {
                format!("symbol \"{}\" was not found", symbol_name)
            }
            Self::CannotReadProperty {
                class_name,
                property_name,
                ..
            } => format!(
                "cannot read property \"{}\" of instance of \"{}\"",
                property_name, class_name
            ),
            Self::InvalidBinaryOperands {
                type_name_left,
                type_name_right,
                ..
            } => format!(
                "invalid operands to binary expression (\"{}\" and \"{}\")",
                type_name_left, type_name_right
            ),
            Self::SymbolNotCallable { symbol_name, .. } => {
                format!("instance of type \"{}\" is not callable", symbol_name)
            }
            Self::InvalidInstantiation { symbol_name, .. } => {
                format!("symbol \"{}\" cannot be instantiated", symbol_name)
            }
        }
    }

    fn span(&self) -> Span {
        match self {
            Self::SymbolNotFound { span, .. } => span.clone(),
            Self::CannotReadProperty { span, .. } => span.clone(),
            Self::InvalidBinaryOperands { span, .. } => span.clone(),
            Self::SymbolNotCallable { span, .. } => span.clone(),
            Self::InvalidInstantiation { span, .. } => span.clone(),
        }
    }

    fn stack_trace(&self) -> &StackTrace {
        match self {
            Self::SymbolNotFound { stack_trace, .. } => stack_trace,
            Self::CannotReadProperty { stack_trace, .. } => stack_trace,
            Self::InvalidBinaryOperands { stack_trace, .. } => stack_trace,
            Self::SymbolNotCallable { stack_trace, .. } => stack_trace,
            Self::InvalidInstantiation { stack_trace, .. } => stack_trace,
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self.span().start().line();
        let column = self.span().start().column();
        let span = self.span();
        let line_content = span.source_code().code().lines().nth(line - 1).unwrap();
        let mut output = format!(
            "{}: {} \
            \n{} {}:{}:{} \
            \n{: >5} {} \
            \n{: >5} {} {} \
            \n{: >5} {}{}{} \
            ",
            "runtime error".red().bold(),
            self.message(),
            "-->".blue().bold(),
            self.span().source_code().file_path(),
            line,
            column,
            " ",
            "|".blue().bold(),
            line.to_string().blue().bold(),
            "|".blue().bold(),
            line_content,
            " ",
            "|".blue().bold(),
            " ".repeat(column),
            "^".repeat(self.span().end().column() - self.span().start().column())
                .red()
                .bold(),
        );
        let mut prev_span = self.span().clone();
        for trace in self.stack_trace().iter().rev() {
            let line = prev_span.start().line();
            let column = prev_span.start().column();
            let function_and_file = format!(
                "{}:{}:{}",
                prev_span.source_code().file_path(),
                line,
                column
            );
            if let Some(class_name) = trace.class_name.clone() {
                output.push_str(&format!(
                    "\n{} {}.{} {}",
                    "at".black(),
                    class_name,
                    trace.function_name.underline(),
                    function_and_file.black()
                ))
            } else {
                output.push_str(&format!(
                    "\n{} {} {}",
                    "at".black(),
                    trace.function_name.underline(),
                    function_and_file.black()
                ));
            }
            prev_span = trace.span.clone();
        }
        let line = prev_span.start().line();
        let column = prev_span.start().column();
        let function_and_file = format!(
            "{}:{}:{}",
            prev_span.source_code().file_path(),
            line,
            column
        );
        output.push_str(&format!("\n{} {}", "at".black(), function_and_file.black()));
        write!(f, "{}", output)
    }
}
