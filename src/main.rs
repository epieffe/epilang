#[macro_use]
extern crate lalrpop_util;

mod compiler;
mod runtime;

use crate::runtime::executor::evalutate_expression;
use crate::runtime::frame::Frame;
use clap::Parser;
use std::fs;

#[derive(Debug, Parser)]
#[clap(name = "lr language interpreter", about, verbatim_doc_comment)]
struct Args {
    #[clap(short, long)]
    program_file: String,
}

fn main() {
    let args: Args = Args::parse();
    println!("Running program {}", args.program_file);

    let program_text = fs::read_to_string(args.program_file)
        .expect("Unable to read the program file");

    let program = compiler::lr_lang::ExprParser::new()
        .parse(&program_text)
        .expect("Unable to parse the program file");

    let frame = Frame::default();
    let (value, frame) = evalutate_expression(frame, &program).unwrap();
    println!("Value: {:#?}", value);
    println!("Main frame: {:#?}", frame);
}
