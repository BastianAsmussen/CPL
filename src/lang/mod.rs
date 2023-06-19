use std::io::Write;

use crate::util::timer::{format_time, Timer};

pub mod errors;
pub mod generator;
pub mod lexer;
pub mod parser;

/// The maximum number of parameters a function can have.
pub const MAX_PARAMETERS: usize = 255;
/// The maximum number of arguments a function can take.
pub const MAX_ARGUMENTS: usize = 255;

/// A struct representing a CPL program.
pub struct Cpl {
    pub had_error: bool,
}

impl Cpl {
    /// Creates a new CPL program.
    pub fn new() -> Self {
        Self { had_error: false }
    }

    /// Runs the CPL program.
    pub fn run_file(&mut self, file_path: &str) {
        let source = std::fs::read_to_string(file_path).expect("Failed to read file!");

        self.run(source);
    }

    /// Runs the CPL program in REPL mode.
    pub fn run_repl(&mut self) {
        loop {
            // Send the prompt.
            print!("> ");
            // Flush the prompt.
            std::io::stdout().flush().unwrap();

            // Read the input.
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line!");

            if input.trim().to_lowercase() == "exit" {
                println!("Exiting REPL...");
                break;
            }

            self.run(input);
        }
    }

    /// Runs the CPL program.
    ///
    /// # Arguments
    /// * `source` - The source code to run.
    pub fn run(&mut self, source: String) {
        let mut timer = Timer::new();

        // Tokenize the source code.
        println!("Tokenizing...");
        let (time, tokens) = timer.time(|| lexer::Scanner::new(&source).scan_tokens());

        println!("Tokens:\n{:#?}", tokens);
        println!("Tokenization took {}.", format_time(time));

        // Parse the tokens.
        println!("Parsing...");
        let (time, syntax_tree) = timer.time(|| parser::Parser::new(&tokens).parse());

        println!("Syntax tree:\n{:#?}", syntax_tree);
        println!("Parsing took {}.", format_time(time));

        if syntax_tree.is_err() {
            self.had_error = true;
            return;
        }

        // Generate the assembly code.
        //println!("Generating code...");
        //let (time, assembly) =
        //    timer.time(|| generator::Generator::new(syntax_tree.unwrap()).generate());

        //println!("Assembly:\n{}", assembly);
        //println!("Code generation took {}.", format_time(time));

        println!("Total time: {}.", format_time(timer.total_time()));
    }
}

impl Default for Cpl {
    fn default() -> Self {
        Self::new()
    }
}
