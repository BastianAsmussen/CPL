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

    println!("Source Code:\n{}", source);

    /*
    Source Code
    -> Tokens
    -> Abstract Syntax Tree (AST)
    -> Semantic Analysis (Type Checking)
    -> Interpreter
     */

    // Tokenize the source code.
    let tokens = lang::lexer::tokenize(&source);

    println!("Tokens:\n{:#?}", tokens);

    // Parse the tokens into an AST.
    let ast = lang::parser::parse(tokens);
    match ast {
        Ok(expr) => println!("AST:\n{:#?}", expr),
        Err(error) => println!("Parsing Error: {}", error),
    }
}
