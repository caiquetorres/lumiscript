use super::line_column::LineColumn;
use super::source_code::SourceCode;

/// The `Span` represents a portion of the source code, restricted by the
/// `start` and `end` fields.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    start: LineColumn,
    end: LineColumn,
    source_code: SourceCode,
}

impl Span {
    /// Creates a new `SourceCode` instance.
    ///
    /// # Arguments
    /// * `start` - The start line column
    /// * `end` - The end line column
    /// * `source_code` - The source code.
    pub fn new(start: LineColumn, end: LineColumn, source_code: SourceCode) -> Self {
        Self {
            start,
            end,
            source_code,
        }
    }

    pub fn from(span: &Self) -> Self {
        Self {
            start: span.start,
            end: span.end,
            source_code: span.source_code.clone(),
        }
    }

    pub fn range(start: &Self, end: &Self) -> Self {
        Self {
            start: start.start,
            end: end.end,
            source_code: start.source_code.clone(),
        }
    }

    /// Gets the span start line column.
    pub fn start(&self) -> LineColumn {
        self.start
    }

    /// Gets the span end line column.
    pub fn end(&self) -> LineColumn {
        self.end
    }

    /// Gets the span text.
    pub fn source_text(&self) -> String {
        String::from(&self.source_code[self.start..self.end])
    }

    pub fn source_code(&self) -> &SourceCode {
        &self.source_code
    }
}

#[macro_export]
macro_rules! span {
    ($t:ident) => {
        impl $t {
            pub fn span(&self) -> &Span {
                &self.span
            }
        }
    };
}
