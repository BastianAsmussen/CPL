use crate::lang::repl::start_repl;
use crate::util::files;

mod lang;
mod util;

fn main() {

    let mut timer = util::timer::Timer::new();

    // Get the file passed as the first argument.
    let is_source_code_provided= std::env::args().nth(1).is_some();

    if !is_source_code_provided {
        println!("No file specified, starting REPL...");
        start_repl();

        return;
    }

    let file = std::env::args().nth(1).unwrap();
    if !files::is_valid_file(&file) {
        return;
    }

    // Read the file into a string.
    println!("Reading file...");
    let (time, source) = timer.time(||
        std::fs::read_to_string(file)
            .expect("Failed to read file!")
    );

    println!("Source Code:\n{}", source);
    println!("Reading file took {}.", util::timer::format_time(time));
    /*
    Source Code
    -> Tokens
    -> Abstract Syntax Tree (AST)
    -> Semantic Analysis (Type Checking)
    -> Interpreter
     */

    // Tokenize the source code.
    println!("Tokenizing...");
    let (time, tokens) = timer.time(|| lang::lexer::tokenize(&source));

    println!("Tokens:\n{:#?}", tokens);
    println!("Tokenization took {}.", util::timer::format_time(time));

    // Parse the tokens into an AST.
    println!("Parsing...");
    let (time, ast) = timer.time(|| lang::parser::Parser::new(&tokens).parse());

    println!("AST:\n{:#?}", ast);
    println!("Parsing took {}.", util::timer::format_time(time));

    if ast.is_err() {
        println!("Parsing failed!");

        return;
    }

    // Analyze the AST.
    println!("Analyzing...");
    let (time, semantics) = timer.time(|| lang::semantic_analyzer::Analyzer::analyze(&ast.unwrap()));

    println!("Semantics:\n{:#?}", semantics);
    println!("Analysis took {}.", util::timer::format_time(time));

    println!("Total Time: {}.", util::timer::format_time(timer.total_time()));
}
