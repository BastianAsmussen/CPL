mod lang;

fn main() {
    // Get the file passed as the first argument.
    let file =
        std::env::args()
            .nth(1)
            .expect("No source file provided!");

    // Read the file into a string.
    let source =
        std::fs::read_to_string(file)
            .expect("Failed to read file!");

    println!("Source code:\n{}", source);
}
