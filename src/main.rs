#[macro_use]
extern crate lalrpop_util;

mod compiler;
mod intermediate;
mod runtime;

use std::env;
use std::fs;
use thiserror::Error;
use rustyline::Editor;

use compiler::epilang::EpilangParser;
use compiler::frame::Frame;
use compiler::compiler::compile;
use compiler::error::CompilerError;
use runtime::executor::{ExpressionError, evaluate};
use runtime::value::{Pointer, Value, V};

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("SyntaxError: {0}")]
    SyntaxError(String),
    #[error("CompilerError: {0}")]
    CompilerError(CompilerError),
    #[error("{0}")]
    RuntimeError(ExpressionError),
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_path: String = args.remove(1);
        run_file(file_path)
    } else {
        repl()
    }
}

pub fn run_file(file_path: String) {
    let text = fs::read_to_string(file_path)
        .expect("Unable to read the program file");

    let mut frame: Frame = Default::default();
    let mut stack: Vec<Pointer> = Vec::new();

    match run_program(text, &mut frame, &mut stack) {
        Ok(v) => println!("Result: {}", v.as_ref()),
        Err(e) => println!("{}", e),
    }
}

pub fn repl() {
    let mut frame: Frame = Default::default();
    let mut stack: Vec<Pointer> = Vec::new();

    let mut rl: Editor<()> = Editor::<()>::new().expect("Error creating editor");
    loop {
        match rl.readline("epilang> ") {
            Ok(mut text) => {
                if text.trim().is_empty() { continue };
                rl.add_history_entry(text.as_str());
                while continue_reading(text.as_str()) {
                    match rl.readline("... ") {
                        Ok(next_line) => {
                            rl.add_history_entry(next_line.as_str());
                            text.push('\n');
                            text.push_str(next_line.as_str())
                        },
                        Err(_) => break
                    }
                }
                match run_program(text, &mut frame, &mut stack) {
                    Ok(v) => {
                        match v.as_ref() {
                            Value::Unit => (),
                            value => println!("{}", value)
                        }
                    },
                    Err(e) => println!("{}", e),
                }
            },
            Err(_) => break
        };
    }
}

fn run_program(line: String, frame: &mut Frame, stack: &mut Vec<Pointer>) -> Result<V, ProgramError> {
    let ast = EpilangParser::new().parse(&line)
        .map_err(|e| { ProgramError::SyntaxError(e.to_string()) })?;

    let exp = compile(&ast, frame)
        .map_err(|e| { ProgramError::CompilerError(e) })?;

    let v = evaluate(&exp, stack, 0)
        .map_err(|e| { ProgramError::RuntimeError(e) })?;

    Ok(v)
}

/// Used in the REPL to check if some string is ready to be evaluated
/// or if the REPL must continue reading
fn continue_reading(text: &str) -> bool {
    if text.trim_end().ends_with('.') {
        return true
    }
    let mut round_brackets_count = 0;
    let mut square_brackets_count = 0;
    let mut curly_brackets_count = 0;
    for c in text.chars() {
        match c {
            '(' => round_brackets_count += 1,
            '[' => square_brackets_count += 1,
            '{' => curly_brackets_count += 1,
            ')' => round_brackets_count -= 1,
            ']' => square_brackets_count -= 1,
            '}' => curly_brackets_count -= 1,
            _ => ()
        }
    }
    round_brackets_count > 0 || square_brackets_count > 0 || curly_brackets_count > 0
}
