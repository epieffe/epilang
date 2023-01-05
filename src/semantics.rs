use std::mem;

use crate::expression::Exp;
use crate::parser::SyntaxError;
use crate::value::{Value, StackValue, Function};

use Value::Int;
use Value::Bool;

pub struct Error {
    pub msg: String
}

impl Error {
    pub fn new(msg: String) -> Self {
        Error {msg: msg}
    }
}

pub fn eval(exp: &Exp) -> Result<Value, Error> {
    let mut stack: Vec<StackValue> = Vec::new();
    eval_expression(exp, &mut stack)
}

pub fn eval_expression(exp: &Exp, stack: &mut Vec<StackValue>) -> Result<Value, Error> {
    match exp {
        Exp::Const(c) => Result::Ok(Value::from_const(c)),

        Exp::Var(x) => Result::Ok(stack[x.scope].read_value()),

        Exp::Decl(_, val_exp, exp2) => {
            let val: Value = eval_expression(val_exp, stack)?;
            stack.push(StackValue::from_box(Box::new(val)));
            let result = eval_expression(exp2, stack);
            stack.pop();
            result
        },

        Exp::Assign(var, exp) => {
            match &**exp {
                Exp::Var(var2) => stack[var.scope] = stack[var2.scope],
                exp => {
                    let val = eval_expression(&exp, stack)?;
                    stack[var.scope] = StackValue::from_box(Box::new(val))
                }
            }
            Result::Ok(Value::Unit)
        }

        Exp::IfThenElse(condition, exp1, exp2) => {
            let branch: &Exp = match eval_expression(condition, stack) {
                // If condition is false evaluate exp2
                Result::Ok(Bool(false) | Int(0)) => exp2,
                // else evaluate exp1
                Result::Ok(_) => exp1,
                Result::Err(err) => return Result::Err(err)
            };
            eval_expression(branch, stack)
        }

        Exp::Function(args, body) => {
            Result::Ok(Value::Fn(Function { num_args: args.len(), external_values: Vec::new(), body: &**body }))
        },
        Exp::FunctionCall(callable, args) => {
            match &eval_expression(callable, stack)? {
                Value::Fn(function) => {
                    if args.len() != function.num_args {
                        return Result::Err(Error { msg: format!("Wrong number of arguments. Expected {}, found {}", function.num_args, args.len()) })
                    }
                    for arg in args {
                        let value: Value = eval_expression(arg, stack)?;
                        stack.push(StackValue::from_box(Box::new(value)))
                    };
                    let result: Value = eval_expression(unsafe{&*function.body}, stack)?;
                    for _ in 0..function.num_args {
                        stack.pop();
                    }
                    Result::Ok(result)
                },
                _ => return Result::Err(Error{msg: String::from("Expression is not callable")})
            }
        },

        Exp::Seq(exp1, exp2) => {
            eval_expression(exp1, stack)?;
            eval_expression(exp2, stack)
        },

        Exp::Sum(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack)?;
            match (val1, val2) {
                (Int(i1), Int(i2)) => Result::Ok(Int(i1 + i2)),
                (Value::Str(i1), Value::Str(i2)) => Result::Ok(Value::Str(i1 + &i2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported + operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Sub(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack)?;
            match (val1, val2) {
                (Int(i1), Int(i2)) => Result::Ok(Int(i1 - i2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported - operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Mul(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack)?;
            match (val1, val2) {
                (Int(i1), Int(i2)) => Result::Ok(Int(i1 * i2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported * operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Div(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack)?;
            match (val1, val2) {
                (Int(i1), Int(i2)) => Result::Ok(Int(i1 / i2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported / operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Lt(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack)?;
            match (val1, val2) {
                (Int(i1), Int(i2)) => Result::Ok(Bool(i1 < i2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported < operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Gt(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack)?;
            match (val1, val2) {
                (Int(i1), Int(i2)) => Result::Ok(Bool(i1 > i2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported > operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Eq(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack)?;
            match (val1, val2) {
                (Int(i1), Int(i2)) => Result::Ok(Bool(i1 == i2)),
                (Bool(b1), Bool(b2)) => Result::Ok(Bool(b1 == b2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported == operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Neq(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack)?;
            match (val1, val2) {
                (Int(i1), Int(i2)) => Result::Ok(Bool(i1 != i2)),
                (Bool(b1), Bool(b2)) => Result::Ok(Bool(b1 != b2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported != operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::And(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack)?;
            match (val1, val2) {
                (Bool(b1), Bool(b2)) => Result::Ok(Bool(b1 && b2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported && operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Or(exp1, exp2) => {
            let (val1, val2) = double_eval(exp1, exp2, stack)?;
            match (val1, val2) {
                (Bool(b1), Bool(b2)) => Result::Ok(Bool(b1 || b2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported || operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Not(exp1) => {
            let val: Value = eval_expression(exp1, stack)?;
            match val {
                Bool(val) => Result::Ok(Bool(!val)),
                v => return Result::Err(Error{
                    msg: format!("Unsupported ! operator for value {}", v)
                })
            }
        }
    }
}

fn double_eval(exp1: &Exp, exp2: &Exp, stack: &mut Vec<StackValue>) -> Result<(Value, Value), Error> {
    let val1 = eval_expression(exp1, stack)?;
    let val2 = eval_expression(exp2, stack)?;
    Result::Ok((val1, val2))
}
