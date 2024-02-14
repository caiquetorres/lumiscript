/// The `LineColumn` stores the line, the column and the index of a
/// certain source code. It is useful for showing error messages and
/// warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineColumn {
    line: usize,
    column: usize,
    index: usize,
}

impl LineColumn {
    /// Gets the line.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Gets the column.
    pub fn column(&self) -> usize {
        self.column
    }

    /// Gets the index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Increases the column value in 1 and the index value also to 1.
    pub(crate) fn next_column(&mut self) {
        self.column += 1;
        self.index += 1;
    }

    /// Increases the line and the index values in 1 and resets the column
    /// to its default value (1).
    pub(crate) fn next_line(&mut self) {
        self.line += 1;
        self.column = 1;
        self.index += 1;
    }
}

impl Default for LineColumn {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            index: 0,
        }
    }
}
