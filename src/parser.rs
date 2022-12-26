use crate::expression::Exp;
use crate::expression::Const;

use crate::expression::Var;
use crate::token::Token;
use crate::token::Operator;

pub fn parse(tokens: &mut Vec<Token>) -> Result<Exp, Error>  {
    match parse_tokens(tokens, vec![], 0, Option::None) {
        Result::Ok((exp, _)) => Result::Ok(exp),
        Result::Err(err) => Result::Err(err)
    }
}

pub enum Error {
    SyntaxError,
}

fn parse_tokens(
    tokens: &mut Vec<Token>,
    stop_tokens: Vec<Token>,
    mut scope: u32,
    stop_on_scope: Option<u32>,
) -> Result<(Exp, Option<Token>), Error> {

    let mut stack: Vec<Token> = Vec::new();
    let mut out: Vec<Exp> = Vec::new();

    let mut token_opt: Option<Token>;
    loop {
        token_opt = tokens.pop();
        // Stop parsing if current token is a stop token or there are no more tokens
        let token = match &token_opt {
            Option::None => if stop_tokens.is_empty() {
                break
            } else {
                return Result::Err(Error::SyntaxError)
            }
            Option::Some(token) => if stop_tokens.contains(token) {
                break
            } else {
                token
            }
        };

        match token {
            Token::Operand(o) => out.push(o.to_exp()),
            Token::Operator(op) => match handle_operator_token(op, &mut stack, &mut out) {
                Result::Ok(_) => (),
                Result::Err(err) => return Result::Err(err)
            },
            Token::RoundBracketOpen => stack.push(Token::RoundBracketOpen),
            Token::RoundBracketClosed => match handle_round_bracket_closed_token(&mut stack, &mut out) {
                Result::Ok(_) => (),
                Result::Err(err) => return Result::Err(err)
            },
            Token::If => match handle_if_token(tokens, &mut out, scope, stop_on_scope) {
                Result::Ok(_) => (),
                Result::Err(err) => return Result::Err(err)
            },
            Token::Let => match handle_let_token(tokens, &mut out, scope) {
                Result::Ok(_) => (),
                Result::Err(err) => return Result::Err(err)
            },
            Token::CurlyBracketOpen => {
                let exp: Exp = match parse_tokens(tokens, vec![], scope + 1, Option::Some(scope)) {
                    Result::Ok((exp, _)) => exp,
                    Result::Err(err) => return Result::Err(err)
                };
                out.push(exp)
            }
            Token::CurlyBracketClosed => {
                scope -= 1;
                match stop_on_scope {
                    Option::None => (),
                    Option::Some(s) => if s <= scope {
                        break
                    }
                }
            }
            Token::Else => return Result::Err(Error::SyntaxError)
        }
    }

    match final_process(&mut stack, &mut out) {
        Result::Ok(exp) => Result::Ok((exp, token_opt)),
        Result::Err(err) => Result::Err(err)
    }
}

fn final_process(stack: &mut Vec<Token>, out: &mut Vec<Exp>) -> Result<Exp, Error> {
    loop {
        match stack.pop() {
            Option::None => break,
            Option::Some(Token::RoundBracketOpen) => return Result::Err(Error::SyntaxError),
            Option::Some(Token::Operator(op)) => {
                let result: Result<(), Error> = push_operator_to_out(&op, out);
                match result {
                    Result::Ok(()) => (),
                    Result::Err(err) => return Result::Err(err)
                };
            }
            Option::Some(Token::CurlyBracketOpen | Token::CurlyBracketClosed) => return Result::Err(Error::SyntaxError),
            Option::Some(Token::RoundBracketClosed | Token::Operand(_) | Token::Let | Token::If | Token::Else) => panic!("This can never happer")
        }
    }
    
    if out.len() != 1 {
        Result::Err(Error::SyntaxError)
    } else {
        Result::Ok(out.pop().unwrap())
    }
}

fn handle_if_token(tokens: &mut Vec<Token>, out: &mut Vec<Exp>, scope: u32, stop_on_scope: Option<u32>) -> Result<(), Error> {
    let condition: Exp = match parse_tokens(tokens, vec![Token::CurlyBracketOpen], scope, Option::None) {
        Result::Ok((exp, _)) => exp,
        Result::Err(e) => return Result::Err(e)
    };

    let if_branch: Exp = match parse_tokens(tokens, vec![Token::CurlyBracketClosed], scope + 1, Option::None) {
        Result::Ok((exp, _)) => exp,
        Result::Err(e) => return Result::Err(e)
    };

    let else_branch: Exp = match tokens.last() {
        Option::Some(Token::Else) => {
            tokens.pop();
            match tokens.pop() {
                Option::Some(Token::CurlyBracketOpen) => match parse_tokens(tokens, vec![Token::CurlyBracketClosed], scope, stop_on_scope) {
                    Result::Ok((exp, _)) => exp,
                    Result::Err(err) => return Result::Err(err)
                }
                _ => return Result::Err(Error::SyntaxError)
            }
        }
        _ => Exp::Const(Const::None)
    };

    out.push(Exp::IfThenElse(Box::new(condition), Box::new(if_branch), Box::new(else_branch)));
    Result::Ok(())
}

fn handle_let_token(tokens: &mut Vec<Token>, out: &mut Vec<Exp>, scope: u32) -> Result<(), Error> {
    let (var, last_token) = match parse_tokens(tokens, vec![Token::Operator(Operator::Assign), Token::Operator(Operator::Seq)], scope, Option::None) {
        Result::Ok((Exp::Var(x), last_token)) => (x, last_token),
        Result::Ok((_, _)) => return Result::Err(Error::SyntaxError),
        Result::Err(err) => return Result::Err(err)
    };
    let value: Exp = match last_token {
        Option::Some(Token::Operator(Operator::Assign)) => {
            match parse_tokens(tokens, vec![Token::Operator(Operator::Seq)], scope, Option::None) {
                Result::Ok((exp, _)) => exp,
                Result::Err(err) => return Result::Err(err)
            }
        },
        Option::Some(Token::Operator(Operator::Seq)) => Exp::Const(Const::None),
        _ => panic!("this can never happen")
    };
    let exp: Exp = match parse_tokens(tokens, vec![], scope + 1, Option::Some(scope)) {
        Result::Ok((exp, last_token)) => {
            match last_token {
                // If the last processed token was a closed curly bracket we need to process it again
                Option::Some(Token::CurlyBracketClosed) => tokens.push(Token::CurlyBracketClosed),
                Option::None => (),
                Option::Some(_) => panic!("this can never happen")
            };
            exp
        }
        Result::Err(err) => return Result::Err(err)
    };
    out.push(Exp::Decl(var, Box::new(value), Box::new(exp)));
    Result::Ok(())
}

fn handle_round_bracket_closed_token(stack: &mut Vec<Token>, out: &mut Vec<Exp>) -> Result<(), Error> {
    loop {
        match stack.pop() {
            Option::None => return Result::Err(Error::SyntaxError),
            Option::Some(Token::RoundBracketOpen) => break,
            Option::Some(Token::Operator(op)) => {
                let result: Result<(), Error> = push_operator_to_out(&op, out);
                match result {
                    Result::Ok(()) => (),
                    Result::Err(err) => return Result::Err(err)
                };
            },
            Option::Some(Token::CurlyBracketOpen | Token::CurlyBracketClosed) => return Result::Err(Error::SyntaxError),
            Option::Some(Token::RoundBracketClosed | Token::Operand(_) | Token::Let | Token::If | Token::Else) => panic!("This can never happer")
        }
    };
    Result::Ok(())
}

fn handle_operator_token(op: &Operator, stack: &mut Vec<Token>, out: &mut Vec<Exp>) -> Result<(), Error> {
    loop {
        match stack.last() {
            Option::None => break,
            Option::Some(Token::RoundBracketOpen) => break,
            Option::Some(Token::Operator(o2)) => {
                if o2.precedence() > op.precedence() {
                    break;
                } else {
                    let result: Result<(), Error> = push_operator_to_out(o2, out);
                    match result {
                        Result::Ok(()) => stack.pop(),
                        Result::Err(err) => return Result::Err(err)
                    };
                }
            },
            Option::Some(Token::CurlyBracketOpen | Token::CurlyBracketClosed) => return Result::Err(Error::SyntaxError),
            Option::Some(Token::RoundBracketClosed | Token::Operand(_) | Token::Let | Token::If | Token::Else) => panic!("This can never happer")
        }
    }
    stack.push(Token::Operator(*op));
    Result::Ok(())
}

fn push_operator_to_out(op: &Operator, out: &mut Vec<Exp>) -> Result<(), Error> {
    match op {
        Operator::Seq => {
            if out.len() < 2 {
                return Result::Err(Error::SyntaxError)
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Seq(Box::new(o1), Box::new(o2)))
        },
        Operator::Assign => {
            if out.len() < 2 {
                return Result::Err(Error::SyntaxError)
            }
            let (exp, var) = (out.pop().unwrap(), out.pop().unwrap());
            match var {
                Exp::Var(var) => out.push(Exp::Assign(var, Box::new(exp))),
                _ => return Result::Err(Error::SyntaxError)
            }
        },
        Operator::Mul => {
            if out.len() < 2 {
                return Result::Err(Error::SyntaxError)
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Mul(Box::new(o1), Box::new(o2)))
        },
        Operator::Div => {
            if out.len() < 2 {
                return Result::Err(Error::SyntaxError)
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Div(Box::new(o1), Box::new(o2)))
        },
        Operator::Sum => {
            if out.len() < 2 {
                return Result::Err(Error::SyntaxError)
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Sum(Box::new(o1), Box::new(o2)))
        },
        Operator::Sub => {
            if out.len() < 2 {
                return Result::Err(Error::SyntaxError)
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Sub(Box::new(o1), Box::new(o2)))
        },
        Operator::Lt => {
            if out.len() < 2 {
                return Result::Err(Error::SyntaxError)
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Lt(Box::new(o1), Box::new(o2)))
        },
        Operator::Gt => {
            if out.len() < 2 {
                return Result::Err(Error::SyntaxError)
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Gt(Box::new(o1), Box::new(o2)))
        },
        Operator::Eq => {
            if out.len() < 2 {
                return Result::Err(Error::SyntaxError)
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Eq(Box::new(o1), Box::new(o2)))
        },
        Operator::And => {
            if out.len() < 2 {
                return Result::Err(Error::SyntaxError)
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::And(Box::new(o1), Box::new(o2)))
        },
        Operator::Or => {
            if out.len() < 2 {
                return Result::Err(Error::SyntaxError)
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Or(Box::new(o1), Box::new(o2)))
        },
        Operator::Not => {
            if out.len() < 1 {
                return Result::Err(Error::SyntaxError)
            }
            let o = out.pop().unwrap();
            out.push(Exp::Not(Box::new(o)))
        },
    }
    Result::Ok(())
}
