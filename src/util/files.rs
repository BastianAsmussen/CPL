use std::path::Path;

/// The file extension for the source code files.
pub const FILE_EXTENSION: &str = "cpl";

/// Checks if the given file is a valid source code file.
///
/// # Arguments
/// * `file` - The path to the file to check.
///
/// # Returns
/// True if the given file is a valid source code file, false otherwise.
pub fn is_valid_file(file: &str) -> bool {
    let path = Path::new(file);
    if !path.exists() {
        eprintln!("File '{}' does not exist!", file);

        return false;
    }

    if !path.is_file() {
        eprintln!("'{}' is not a file!", file);

        return false;
    }

    if path.extension().unwrap() != FILE_EXTENSION {
        eprintln!("File '{}' must have '.{}' extension!", file, FILE_EXTENSION);

        return false;
    }

    true
}
