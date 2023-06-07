use crate::lang::parser::Parser;
use crate::util::timer;

mod lang;
mod util;

fn main() {

    // Get the file passed as the first argument.
    let file =
        std::env::args()
            .nth(1)
            .expect("No source file provided!");

    // Read the file into a string.
    println!("Reading file...");

    let (time, source) = timer::time(||
        std::fs::read_to_string(file)
            .expect("Failed to read file!")
    );

    println!("Source Code:\n{}", source);
    println!("Reading file took {}.", timer::format_time(time));
    /*
    Source Code
    -> Tokens
    -> Abstract Syntax Tree (AST)
    -> Semantic Analysis (Type Checking)
    -> Interpreter
     */

    // Tokenize the source code.
    println!("Tokenizing...");
    let (time, tokens) = timer::time(|| lang::lexer::tokenize(&source));

    println!("Tokens:\n{:#?}", tokens);
    println!("Tokenization took {}.", timer::format_time(time));

    // Parse the tokens into an AST.
    println!("Parsing...");
    let (time, ast) = timer::time(|| Parser::new(&tokens).parse());

    println!("AST:\n{:#?}", ast);
    println!("Parsing took {}.", timer::format_time(time));
}
