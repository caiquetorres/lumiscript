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
}
