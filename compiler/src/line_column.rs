/// `LineColumn` is a basic structure denoting a position within the source
/// code with line and column information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
}
