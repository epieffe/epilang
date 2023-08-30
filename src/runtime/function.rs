use std::io::{self, Write};

use crate::intermediate::exp::{Exp, BuiltInFunction};

use super::pointer::Ptr;
use super::executor::ExpressionError;
use super::value::{Value, V};

#[derive(Debug)]
pub struct Function {
    pub num_args: usize,
    pub external_values: Vec<Ptr<Value>>,
    pub body: Exp
}

#[derive(Debug)]
pub struct Method {
    pub self_value: Ptr<Value>,
    pub function: Ptr<Function>
}

impl BuiltInFunction {
    pub fn call(&self, args: Vec<Ptr<Value>>) -> Result<V, ExpressionError> {
        if args.len() != self.num_args() {
            return Err(ExpressionError::WrongArgumentsNumber(self.num_args(), args.len()))
        }
        match self {
            BuiltInFunction::Print => {
                print!("{}", args[0]);
                Ok(V::Ptr(Ptr::unit()))
            },

            BuiltInFunction::Println => {
                match args[0].as_ref() {
                    Value::String(s) => println!("{}", s),
                    v => println!("{}", v),
                }
                Ok(V::Ptr(Ptr::unit()))
            },

            BuiltInFunction::Input => {
                print!("{}", args[0]);
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("error: unable to read user input");
                let len = input.trim_end_matches(&['\r', '\n'][..]).len(); // Remove final end of line
                input.truncate(len);
                Ok(V::Val(Value::String(input)))
            },
        }
    }

    fn num_args(&self) -> usize {
        match self {
            BuiltInFunction::Print => 1,
            BuiltInFunction::Println => 1,
            BuiltInFunction::Input => 1,
        }
    }
}
