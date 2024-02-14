use std::fmt::Debug;
use std::fs;
use std::ops::Index;
use std::ops::Range;
use std::rc::Rc;

use crate::utils::line_column::LineColumn;

/// The `SourceCodeError` encapsulates the message of the error
/// intercepted when reading the source file.
#[derive(Debug)]
pub struct SourceCodeError {
    message: String,
}

impl SourceCodeError {
    /// Creates a new `SourceCodeError` instance.
    ///
    /// # Arguments
    /// * `message` - The error message.
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }

    /// Gets the error message.
    pub fn message(&self) -> String {
        self.message.clone()
    }
}

struct InnerSourceCode {
    file_path: String,
    code: String,
}

/// The `SourceCode` struct encapsulates the source code data, such as its
/// content and path in a way that allows the efficient sharing of that
/// information.
#[derive(Clone)]
pub struct SourceCode {
    inner: Rc<InnerSourceCode>,
}

impl SourceCode {
    /// Creates a `SourceCode` instance given its file path.
    ///
    /// # Arguments
    /// * `file_path` - The path to the file containing the source code.
    pub fn from_file(file_path: &str) -> Result<Self, SourceCodeError> {
        fs::read_to_string(file_path)
            .map(|code| Self {
                inner: Rc::new(InnerSourceCode {
                    file_path: file_path.to_owned(),
                    code,
                }),
            })
            .map_err(|_| SourceCodeError::new(&format!("File '{file_path}' not found")))
    }

    /// Gets the source code relative file path.
    pub fn file_path(&self) -> &String {
        &self.inner.file_path
    }

    /// Gets the source code content.
    pub fn code(&self) -> &String {
        &self.inner.code
    }
}

impl PartialEq for SourceCode {
    fn eq(&self, other: &Self) -> bool {
        self.inner.code == other.inner.code && self.inner.file_path == self.inner.file_path
    }
}

impl Index<Range<usize>> for SourceCode {
    type Output = str;

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.inner.code[index]
    }
}

impl Index<Range<LineColumn>> for SourceCode {
    type Output = str;

    fn index(&self, index: Range<LineColumn>) -> &Self::Output {
        &self.inner.code[index.start.index()..index.end.index()]
    }
}

impl Debug for SourceCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceCode")
            .field("file_path", &self.inner.file_path)
            .field("code", &self.inner.code)
            .finish()
    }
}
