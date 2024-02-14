use std::ops::Index;
use std::slice::Iter;

use crate::token::Token;

#[derive(Debug, Clone)]
pub struct TokenStream {
    stream: Vec<Token>,
}

impl TokenStream {
    pub fn new(stream: &[Token]) -> Self {
        Self {
            stream: stream.to_vec(),
        }
    }

    /// Returns an iterator over the stream.
    pub fn iter(&self) -> Iter<Token> {
        self.stream.iter()
    }
}

impl Index<usize> for TokenStream {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        &self.stream[index]
    }
}

impl Extend<Token> for TokenStream {
    fn extend<T: IntoIterator<Item = Token>>(&mut self, iter: T) {
        self.stream.extend(iter)
    }
}
