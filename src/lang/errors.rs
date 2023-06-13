/// A struct representing an error.
#[derive(Debug, Clone)]
pub struct Error {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

/// Prints an error message to the `stderr` file descriptor.
pub fn report(line: usize, column: usize, message: &str) {
    eprintln!("[line {}:{}]: {}", line, column, message);
}
