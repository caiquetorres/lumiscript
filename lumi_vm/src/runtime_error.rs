#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}
