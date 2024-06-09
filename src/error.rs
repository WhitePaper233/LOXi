use core::fmt;

pub struct Error {
    line: usize,
    index: usize,
    message: String
}

impl Error {
    pub fn new(line: usize, index: usize, message: &str) -> Self {
        return Error{
            line,
            index,
            message: message.to_string(),
        };
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}] {}\n", self.line, self.index, self.message)
    }
}