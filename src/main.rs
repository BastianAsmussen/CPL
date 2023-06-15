use crate::util::files;

mod lang;
mod util;

fn main() {
    let mut cpl = lang::Cpl::new();

    // Get the file passed as the first argument.
    let is_source_code_provided = std::env::args().nth(1).is_some();
    if !is_source_code_provided {
        println!("No file specified, starting REPL...");
        cpl.run_repl();

        return;
    }

    let file_path = std::env::args().nth(1).unwrap();
    if !files::is_valid_file(&file_path) {
        return;
    }

    cpl.run_file(&file_path);
}
