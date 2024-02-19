#[derive(Debug)]
pub struct RuntimeError {
    _message: String,
}

impl RuntimeError {
    pub fn new(message: &str) -> Self {
        Self {
            _message: message.to_owned(),
        }
    }
}
