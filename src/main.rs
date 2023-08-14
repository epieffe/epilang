#[macro_use]
extern crate lalrpop_util;

mod compiler;
mod intermediate;
mod runtime;

use clap::Parser;
use std::fs;
use thiserror::Error;
use compiler::compiler::CompilerError;
use runtime::executor::ExpressionError;


#[derive(Debug, Parser)]
#[clap(name = "lr language interpreter", about, verbatim_doc_comment)]
struct Args {
    #[clap(short, long)]
    program_file: String,
}

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("CompilerError: {0}")]
    CompilerError(CompilerError),
    #[error("RuntimeError: {0}")]
    ExpressionError(ExpressionError),
}

fn main() {
    let args: Args = Args::parse();
    println!("Running program {}", args.program_file);

    let program_text = fs::read_to_string(args.program_file)
        .expect("Unable to read the program file");

    let ast = compiler::lr_lang::ASTParser::new()
        .parse(&program_text)
        .expect("Unable to parse the program file");

    let exp = compiler::compiler::compile(ast.as_ref()).unwrap_or_else(|e| {
        panic!("CompilerError: {}", e)
    });

    let v = runtime::executor::evaluate(&exp).unwrap_or_else(|e| {
        panic!("RuntimeError: {}", e)
    });

    println!("Value: {}", v.as_ref());
}
