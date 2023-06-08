use std::io::Write;

pub fn start_repl() {
    loop {
        // Send the prompt.
        print!("> ");
        // Flush the prompt.
        std::io::stdout().flush().unwrap();

        // Read the input.
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        if input.trim().to_lowercase() == "exit" {
            println!("Exiting REPL...");
            break;
        }
        
        // Tokenize the input.
        let tokens = crate::lang::lexer::tokenize(&input);

        // Parse the tokens into an AST.
        let ast = crate::lang::parser::Parser::new(&tokens).parse();

        println!("Tokens:\n{:#?}", tokens);
        println!("AST:\n{:#?}", ast);
    }
}
