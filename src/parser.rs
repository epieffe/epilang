use crate::expression::Exp;
use crate::token::Token;
use crate::token::Operator;


pub fn parse(mut tokens: Vec<Token>) -> Result<Exp, ()> {
    let mut stack: Vec<Token> = Vec::new();
    let mut out: Vec<Exp> = Vec::new();

    loop {
        match tokens.pop() {
            Option::None => break,
            Option::Some(Token::Operand(ref o)) => out.push(o.to_exp()),
            Option::Some(Token::RoundBracketOpen) => stack.push(Token::RoundBracketOpen),
            Option::Some(Token::Operator(ref o1)) => {
                loop {
                    match stack.last() {
                        Option::None => break,
                        Option::Some(Token::RoundBracketOpen) => break,
                        Option::Some(Token::Operator(ref o2)) => {
                            if o2.precedence() > o1.precedence() {
                                break;
                            } else {
                                let result: Result<(), ()> = push_operator_to_out(o2, &mut out);
                                match result {
                                    Result::Ok(()) => stack.pop(),
                                    Result::Err(err) => return Result::Err(err)
                                };
                            }
                        },
                        Option::Some(Token::RoundBracketClosed | Token::Operand(_)) => panic!("This can never happer")
                    }
                }
                stack.push(Token::Operator(*o1));
            },
            Option::Some(Token::RoundBracketClosed) => {
                loop {
                    match stack.pop() {
                        Option::None => return Result::Err(()),
                        Option::Some(Token::RoundBracketOpen) => break,
                        Option::Some(Token::Operator(op)) => {
                            let result: Result<(), ()> = push_operator_to_out(&op, &mut out);
                            match result {
                                Result::Ok(()) => (),
                                Result::Err(err) => return Result::Err(err)
                            };
                        },
                        Option::Some(Token::RoundBracketClosed | Token::Operand(_)) => panic!("This can never happer")
                    }
                }
            }
        }
    }
    loop {
        match stack.pop() {
            Option::None => break,
            Option::Some(Token::RoundBracketOpen) => return Result::Err(()),
            Option::Some(Token::Operator(op)) => {
                let result: Result<(), ()> = push_operator_to_out(&op, &mut out);
                match result {
                    Result::Ok(()) => (),
                    Result::Err(err) => return Result::Err(err)
                };
            }
            Option::Some(Token::RoundBracketClosed | Token::Operand(_)) => panic!("This can never happer")
        }
    }
    Result::Ok(out.pop().unwrap())
}

fn push_operator_to_out(op: &Operator, out: &mut Vec<Exp>) -> Result<(), ()> {
    match op {
        Operator::Seq => {
            if out.len() < 2 {
                return Result::Err(())
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Seq(Box::new(o1), Box::new(o2)))
        },
        Operator::Assign => {
            if out.len() < 2 {
                return Result::Err(())
            }
            let (exp, var) = (out.pop().unwrap(), out.pop().unwrap());
            match var {
                Exp::Var(var) => out.push(Exp::Assign(var, Box::new(exp))),
                _ => return Result::Err(())
            }
        },
        Operator::Mul => {
            if out.len() < 2 {
                return Result::Err(())
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Mul(Box::new(o1), Box::new(o2)))
        },
        Operator::Div => {
            if out.len() < 2 {
                return Result::Err(())
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Div(Box::new(o1), Box::new(o2)))
        },
        Operator::Sum => {
            if out.len() < 2 {
                return Result::Err(())
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Sum(Box::new(o1), Box::new(o2)))
        },
        Operator::Sub => {
            if out.len() < 2 {
                return Result::Err(())
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Sub(Box::new(o1), Box::new(o2)))
        },
        Operator::Lt => {
            if out.len() < 2 {
                return Result::Err(())
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Lt(Box::new(o1), Box::new(o2)))
        },
        Operator::Gt => {
            if out.len() < 2 {
                return Result::Err(())
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Gt(Box::new(o1), Box::new(o2)))
        },
        Operator::Eq => {
            if out.len() < 2 {
                return Result::Err(())
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Eq(Box::new(o1), Box::new(o2)))
        },
        Operator::And => {
            if out.len() < 2 {
                return Result::Err(())
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::And(Box::new(o1), Box::new(o2)))
        },
        Operator::Or => {
            if out.len() < 2 {
                return Result::Err(())
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Or(Box::new(o1), Box::new(o2)))
        },
        Operator::Not => {
            if out.len() < 1 {
                return Result::Err(())
            }
            let o = out.pop().unwrap();
            out.push(Exp::Not(Box::new(o)))
        },
    }
    Result::Ok(())
}


