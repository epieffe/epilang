use crate::expression::Exp;
use crate::expression::Const;

use Const::Integer;
use Const::Boolean;

pub struct Error {
    pub msg: String
}

impl Error {
    pub fn new(msg: String) -> Self {
        Error {msg: msg}
    }
}

pub fn eval(exp: Exp) -> Result<Const, Error> {
    let mut stack = Vec::new();
    eval_expression(exp, &mut stack)
}

pub fn eval_expression(exp: Exp, stack: &mut Vec<Const>) -> Result<Const, Error> {
    match exp {
        Exp::Const(c) => Result::Ok(c),

        Exp::Var(x) =>Result::Ok(stack[x.scope].clone()),

        Exp::Decl(_, val_exp, exp2) => {
            let val = match eval_expression(*val_exp, stack) {
                Result::Ok(v) => v,
                Result::Err(err) => return Result::Err(err)
            };
            stack.push(val);
            let result = eval_expression(*exp2, stack);
            stack.pop();
            result
        },

        Exp::Assign(var, exp2) => {
            let val = match eval_expression(*exp2, stack) {
                Result::Ok(v) => v,
                Result::Err(err) => return Result::Err(err)
            };
            stack[var.scope] = val;
            Result::Ok(Const::None)
        }

        Exp::IfThenElse(condition, exp1, exp2) => {
            let branch: Exp = match eval_expression(*condition, stack) {
                // If condition is false evaluate exp2
                Result::Ok(Const::Boolean(false) | Const::Integer(0)) => *exp2,
                // else evaluate exp1
                Result::Ok(_) => *exp1,
                Result::Err(err) => return Result::Err(err)
            };
            eval_expression(branch, stack)
        }

        Exp::Seq(exp1, exp2) => {
            match eval_expression(*exp1, stack) {
                Result::Ok(_) => (),
                Result::Err(err) => return Result::Err(err)
            };
            eval_expression(*exp2, stack)
        },

        Exp::Sum(exp1, exp2) => {
            let (val1, val2) = match double_eval(*exp1, *exp2, stack) {
                Result::Ok(values) => values,
                Result::Err(err) => return Result::Err(err)
            };
            match (val1, val2) {
                (Integer(i1), Integer(i2)) => Result::Ok(Integer(i1 + i2)),
                (Const::String(i1), Const::String(i2)) => Result::Ok(Const::String(i1 + &i2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported + operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Sub(exp1, exp2) => {
            let (val1, val2) = match double_eval(*exp1, *exp2, stack) {
                Result::Ok(values) => values,
                Result::Err(err) => return Result::Err(err)
            };
            match (val1, val2) {
                (Integer(i1), Integer(i2)) => Result::Ok(Integer(i1 - i2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported - operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Mul(exp1, exp2) => {
            let (val1, val2) = match double_eval(*exp1, *exp2, stack) {
                Result::Ok(values) => values,
                Result::Err(err) => return Result::Err(err)
            };
            match (val1, val2) {
                (Integer(i1), Integer(i2)) => Result::Ok(Integer(i1 * i2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported * operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Div(exp1, exp2) => {
            let (val1, val2) = match double_eval(*exp1, *exp2, stack) {
                Result::Ok(values) => values,
                Result::Err(err) => return Result::Err(err)
            };
            match (val1, val2) {
                (Integer(i1), Integer(i2)) => Result::Ok(Integer(i1 / i2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported / operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Lt(exp1, exp2) => {
            let (val1, val2) = match double_eval(*exp1, *exp2, stack) {
                Result::Ok(values) => values,
                Result::Err(err) => return Result::Err(err)
            };
            match (val1, val2) {
                (Integer(i1), Integer(i2)) => Result::Ok(Boolean(i1 < i2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported < operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Gt(exp1, exp2) => {
            let (val1, val2) = match double_eval(*exp1, *exp2, stack) {
                Result::Ok(values) => values,
                Result::Err(err) => return Result::Err(err)
            };
            match (val1, val2) {
                (Integer(i1), Integer(i2)) => Result::Ok(Boolean(i1 > i2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported > operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Eq(exp1, exp2) => {
            let (val1, val2) = match double_eval(*exp1, *exp2, stack) {
                Result::Ok(values) => values,
                Result::Err(err) => return Result::Err(err)
            };
            match (val1, val2) {
                (Integer(i1), Integer(i2)) => Result::Ok(Boolean(i1 == i2)),
                (Boolean(b1), Boolean(b2)) => Result::Ok(Boolean(b1 == b2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported == operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::And(exp1, exp2) => {
            let (val1, val2) = match double_eval(*exp1, *exp2, stack) {
                Result::Ok(values) => values,
                Result::Err(err) => return Result::Err(err)
            };
            match (val1, val2) {
                (Boolean(b1), Boolean(b2)) => Result::Ok(Boolean(b1 && b2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported && operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Or(exp1, exp2) => {
            let (val1, val2) = match double_eval(*exp1, *exp2, stack) {
                Result::Ok(values) => values,
                Result::Err(err) => return Result::Err(err)
            };
            match (val1, val2) {
                (Boolean(b1), Boolean(b2)) => Result::Ok(Boolean(b1 || b2)),
                (v1, v2) => return Result::Err(Error{
                    msg: format!("Unsupported || operator for values {}, {}", v1, v2)
                })
            }
        },

        Exp::Not(exp1) => {
            let val: Const = match eval_expression(*exp1, stack) {
                Result::Ok(val) => val,
                Result::Err(err) => return Result::Err(err)
            };
            match val {
                Boolean(val) => Result::Ok(Boolean(!val)),
                v => return Result::Err(Error{
                    msg: format!("Unsupported && operator for value {}", v)
                })
            }
        }
    }
}

fn double_eval(exp1: Exp, exp2: Exp, stack: &mut Vec<Const>) -> Result<(Const, Const), Error> {
    let val1 = match eval_expression(exp1, stack) {
        Result::Ok(val) => val,
        Result::Err(err) => return Result::Err(err)
    };
    let val2 = match eval_expression(exp2, stack) {
        Result::Ok(val) => val,
        Result::Err(err) => return Result::Err(err)
    };
    Result::Ok((val1, val2))
}
