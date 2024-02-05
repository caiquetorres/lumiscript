/// `LineColumn` is a basic structure denoting a position within the source
/// code with line and column information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
}

impl Default for LineColumn {
    /// Creates a default `LineColumn` instance with line 1 and column 1.
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

impl LineColumn {
    /// Moves to the next column in the source code.
    ///
    /// # Example
    ///
    /// ```
    /// let mut position = LineColumn::default();
    /// position.next_column();
    /// ```
    pub fn next_column(&mut self) {
        self.column += 1;
    }

    /// Moves to the next line in the source code, resetting the column to 1.
    ///
    /// # Example
    ///
    /// ```
    /// let mut position = LineColumn::default();
    /// position.next_line();
    /// ```
    pub fn next_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}
