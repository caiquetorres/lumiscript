use std::rc::Rc;

/// `SourceCode` is a simple structure designed to encapsulate source code
/// text in a way that allows for efficient sharing and cloning through
/// reference counting.
#[derive(Debug, Clone)]
pub struct SourceCode {
    code: Rc<String>,
}

impl SourceCode {
    /// Creates a new `SourceCode` instance with the given code.
    ///
    /// The input code is trimmed to remove leading and trailing whitespaces.
    pub fn new(code: &str) -> Self {
        Self {
            code: Rc::new(code.trim().to_owned()),
        }
    }

    /// Retrieves a reference to the underlying source code string.
    pub fn code(&self) -> &String {
        &self.code
    }
}
