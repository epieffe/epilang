use std::convert::TryInto;
use std::io::{self, Write};

use crate::intermediate::constant::Type;
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

#[derive(Debug)]
pub struct BuiltInMethod {
    pub self_value: Ptr<Value>,
    pub function: BuiltInFunction
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
            }

            BuiltInFunction::Println => {
                match args[0].as_ref() {
                    Value::String(s) => println!("{}", s),
                    v => println!("{}", v),
                }
                Ok(V::Ptr(Ptr::unit()))
            }

            BuiltInFunction::Input => {
                print!("{}", args[0]);
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("error: unable to read user input");
                let len = input.trim_end_matches(&['\r', '\n'][..]).len(); // Remove final end of line
                input.truncate(len);
                Ok(V::Val(Value::String(input)))
            }

            BuiltInFunction::ListLength => {
                match args[0].as_ref() {
                    Value::List(list) => {
                        Ok(V::Val(Value::Int(list.len().try_into().unwrap())))
                    },
                    v => Err(ExpressionError::UnexpectedType(Type::List, v.get_type()))
                }
            }

            BuiltInFunction::ListPush => {
                match args[0].clone().as_mut_ref() {
                    Value::List(list) => {
                        list.push(args[1]);
                        Ok(V::Ptr(Ptr::unit()))
                    },
                    v => Err(ExpressionError::UnexpectedType(Type::List, v.get_type()))
                }
            }

            BuiltInFunction::ListPop => {
                match args[0].clone().as_mut_ref() {
                    Value::List(list) => {
                        match list.pop() {
                            Some(v) => Ok(V::Ptr(v)),
                            None => Err(ExpressionError::ListIndexOutofRange),
                        }
                    },
                    v => Err(ExpressionError::UnexpectedType(Type::List, v.get_type()))
                }
            }

            BuiltInFunction::ListRemove => {
                match args[0].clone().as_mut_ref() {
                    Value::List(list) => {
                        match args[1].as_ref() {
                            Value::Int(i) => {
                                let index = (*i).try_into().unwrap();
                                if list.len() > (*i).try_into().unwrap() {
                                    Ok(V::Ptr(list.remove(index)))
                                } else {
                                    Err(ExpressionError::ListIndexOutofRange)
                                }
                            },
                            i => Err(ExpressionError::IndexTypeError(Type::List, i.get_type())),
                        }
                    },
                    v => Err(ExpressionError::UnexpectedType(Type::List, v.get_type()))
                }
            }
        }
    }

    fn num_args(&self) -> usize {
        match self {
            BuiltInFunction::Print => 1,
            BuiltInFunction::Println => 1,
            BuiltInFunction::Input => 1,
            BuiltInFunction::ListLength => 1,
            BuiltInFunction::ListPush => 2,
            BuiltInFunction::ListPop => 1,
            BuiltInFunction::ListRemove => 2,
        }
    }
}
