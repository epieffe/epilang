use crate::expression::Exp;
use crate::expression::Const;
use crate::expression::Var;

pub struct Error {
    msg: String
}

impl Error {
    pub fn new(msg: String) -> Self {
        Error {msg: msg}
    }
}

fn eval_expression(exp: Exp, stack: &mut Vec<Const>) -> Result<Const, Error> {
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
        _ => panic!()
    }
}
