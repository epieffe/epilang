mod expression;
mod token;
mod parser;
mod lexer;
mod semantics;
mod shell;
mod value;
mod run;
mod tests;

use std::env;
use run::run_file;


fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_path: String = args.remove(1);
        run_file(file_path)
    } else {
        shell::run_shell()
    }
}
