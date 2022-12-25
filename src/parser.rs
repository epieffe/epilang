use crate::expression::Exp;
use crate::expression::Const;

use crate::token::Token;
use crate::token::Operator;

pub fn parse(tokens: &mut Vec<Token>) -> Result<Exp, ()>  {
    parse_tokens(tokens, vec![])
}

fn parse_tokens(tokens: &mut Vec<Token>, stop_tokens: Vec<Token>) -> Result<Exp, ()> {
    let mut stack: Vec<Token> = Vec::new();
    let mut out: Vec<Exp> = Vec::new();

    loop {
        // Stop parsing if current token is a stop token or there are no more tokens
        let token = match tokens.pop() {
            Option::None => if stop_tokens.is_empty() {
                break
            } else {
                return Result::Err(())
            }
            Option::Some(token) => if stop_tokens.contains(&token) {
                break
            } else {
                token
            }
        };

        match token {
            Token::Operand(ref o) => out.push(o.to_exp()),
            Token::RoundBracketOpen => stack.push(Token::RoundBracketOpen),
            Token::Operator(ref op) => match handle_operator_token(op, &mut stack, &mut out) {
                Result::Ok(_) => (),
                Result::Err(err) => return Result::Err(err)
            },
            Token::RoundBracketClosed => match handle_round_bracket_closed_token(&mut stack, &mut out) {
                Result::Ok(_) => (),
                Result::Err(err) => return Result::Err(err)
            },
            Token::If => match handle_if_token(tokens, &mut stack, &mut out) {
                Result::Ok(_) => (),
                Result::Err(err) => return Result::Err(err)
            },
            Token::CurlyBracketOpen | Token::CurlyBracketClosed | Token::Else => return Result::Err(())
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
            Option::Some(Token::CurlyBracketOpen | Token::CurlyBracketClosed) => return Result::Err(()),
            Option::Some(Token::RoundBracketClosed | Token::Operand(_) | Token::If | Token::Else) => panic!("This can never happer")
        }
    }
    
    if out.len() != 1 {
        Result::Err(())
    } else {
        Result::Ok(out.pop().unwrap())
    }
}

fn handle_if_token(tokens: &mut Vec<Token>, stack: &mut Vec<Token>, out: &mut Vec<Exp>) -> Result<(), ()> {
    let condition: Exp = match parse_tokens(tokens, vec![Token::CurlyBracketOpen]) {
        Result::Ok(exp) => exp,
        Result::Err(e) => return Result::Err(e)
    };

    let if_branch: Exp = match parse_tokens(tokens, vec![Token::CurlyBracketClosed]) {
        Result::Ok(exp) => exp,
        Result::Err(e) => return Result::Err(e)
    };

    let else_branch: Exp = match tokens.last() {
        Option::Some(Token::Else) => {
            tokens.pop();
            match tokens.pop() {
                Option::Some(Token::CurlyBracketOpen) => match parse_tokens(tokens, vec![Token::CurlyBracketClosed]) {
                    Result::Ok(exp) => exp,
                    Result::Err(err) => return Result::Err(err)
                }
                _ => return Result::Err(())
            }
        }
        _ => Exp::Const(Const::None)
    };

    out.push(Exp::IfThenElse(Box::new(condition), Box::new(if_branch), Box::new(else_branch)));
    Result::Ok(())
}

fn handle_round_bracket_closed_token(stack: &mut Vec<Token>, out: &mut Vec<Exp>) -> Result<(), ()> {
    loop {
        match stack.pop() {
            Option::None => return Result::Err(()),
            Option::Some(Token::RoundBracketOpen) => break,
            Option::Some(Token::Operator(op)) => {
                let result: Result<(), ()> = push_operator_to_out(&op, out);
                match result {
                    Result::Ok(()) => (),
                    Result::Err(err) => return Result::Err(err)
                };
            },
            Option::Some(Token::CurlyBracketOpen | Token::CurlyBracketClosed) => return Result::Err(()),
            Option::Some(Token::RoundBracketClosed | Token::Operand(_) | Token::If | Token::Else) => panic!("This can never happer")
        }
    };
    Result::Ok(())
}

fn handle_operator_token(op: &Operator, stack: &mut Vec<Token>, out: &mut Vec<Exp>) -> Result<(), ()> {
    loop {
        match stack.last() {
            Option::None => break,
            Option::Some(Token::RoundBracketOpen) => break,
            Option::Some(Token::Operator(ref o2)) => {
                if o2.precedence() > op.precedence() {
                    break;
                } else {
                    let result: Result<(), ()> = push_operator_to_out(o2, out);
                    match result {
                        Result::Ok(()) => stack.pop(),
                        Result::Err(err) => return Result::Err(err)
                    };
                }
            },
            Option::Some(Token::CurlyBracketOpen | Token::CurlyBracketClosed) => return Result::Err(()),
            Option::Some(Token::RoundBracketClosed | Token::Operand(_) | Token::If | Token::Else) => panic!("This can never happer")
        }
    }
    stack.push(Token::Operator(*op));
    Result::Ok(())
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


