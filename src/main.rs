#[macro_use]
extern crate lalrpop_util;

mod compiler;
mod intermediate;
mod runtime;
mod error;

use std::env;
use std::fs;
use thiserror::Error;
use rustyline::Editor;

use compiler::lr_lang::ASTParser;
use compiler::compiler::Context;
use compiler::compiler::CompilerError;
use compiler::compiler::compile_with_context;
use runtime::executor::ExpressionError;
use runtime::value::{Pointer, Value, V};
use runtime::executor::evaluate_with_stack;

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("SyntaxError")]
    SyntaxError,
    #[error("CompilerError: {0}")]
    CompilerError(CompilerError),
    #[error("RuntimeError: {0}")]
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

    let mut context: Context = Context::new();
    let mut stack: Vec<Pointer> = Vec::new();

    match run_program(text, &mut context, &mut stack) {
        Ok(v) => println!("Result: {}", v.as_ref()),
        Err(e) => println!("{}", e),
    }
}

pub fn repl() {
    let mut context: Context = Context::new();
    let mut stack: Vec<Pointer> = Vec::new();

    let mut rl: Editor<()> = Editor::<()>::new().expect("Error creating editor");
    loop {
        match rl.readline("epilang> ") {
            Ok(line) => {
                if line.trim().is_empty() { continue };
                rl.add_history_entry(line.as_str());
                match run_program(line, &mut context, &mut stack) {
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

fn run_program(line: String, context: &mut Context, stack: &mut Vec<Pointer>) -> Result<V, ProgramError> {
    let ast = ASTParser::new().parse(&line)
        .map_err(|_| {ProgramError::SyntaxError})?;

    let exp = compile_with_context(ast.as_ref(), context)
        .map_err(|e| {ProgramError::CompilerError(e)})?;

    let v = evaluate_with_stack(&exp, stack, 0)
        .map_err(|e| {ProgramError::RuntimeError(e)})?;

    Ok(v)
}
