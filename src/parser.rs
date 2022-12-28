use std::collections::HashMap;

use crate::expression::Exp;
use crate::expression::Const;
use crate::expression::Var;

use crate::token::Token;
use crate::token::Operand;
use crate::token::Operator;

pub struct SyntaxError {}

pub fn parse(tokens: &mut Vec<Token>) -> Result<Exp, SyntaxError>  {
    let mut variable_scope_map: HashMap<String, usize> = HashMap::new();
    match parse_tokens(tokens, 0, &mut variable_scope_map, vec![], Option::None) {
        Result::Ok((exp, _)) => Result::Ok(exp),
        Result::Err(err) => Result::Err(err)
    }
}

pub fn parse_in_scope(
    tokens: &mut Vec<Token>,
    scope: usize,
    variable_scope_map: &mut HashMap<String, usize>,
) -> Result<Exp, SyntaxError> {
    match parse_tokens(tokens, scope, variable_scope_map, vec![], Option::None) {
        Result::Ok((exp, _)) => Result::Ok(exp),
        Result::Err(err) => Result::Err(err)
    }
}

fn parse_tokens(
    tokens: &mut Vec<Token>,
    scope: usize,
    variable_scope_map: &mut HashMap<String, usize>,
    stop_tokens: Vec<Token>,
    stop_on_scope: Option<usize>,
) -> Result<(Exp, Option<Token>), SyntaxError> {

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
                return Result::Err(SyntaxError{})
            }
            Option::Some(token) => if stop_tokens.contains(token) {
                break
            } else {
                token
            }
        };

        match token {
            // Push variable to out. Error if not present in scope
            Token::Operand(Operand::Var(x)) => match variable_scope_map.get(x) {
                Option::Some(s) => out.push(Exp::Var(Var{name: x.to_string(), scope: *s})),
                Option::None => return Result::Err(SyntaxError{})
            }
            // Operands that are not variables are easily pushed to out
            Token::Operand(o) => out.push(o.to_exp()),
            // Handle operator tokens (eg: +, -, *, /, &&, ||, !)
            Token::Operator(op) => match handle_operator_token(op, &mut stack, &mut out) {
                Result::Ok(_) => (),
                Result::Err(err) => return Result::Err(err)
            },
            Token::RoundBracketOpen => stack.push(Token::RoundBracketOpen),
            Token::RoundBracketClosed => match handle_round_bracket_closed_token(&mut stack, &mut out) {
                Result::Ok(_) => (),
                Result::Err(err) => return Result::Err(err)
            },
            Token::If => match handle_if_token(tokens, &mut out, variable_scope_map, scope, stop_on_scope) {
                Result::Ok(_) => (),
                Result::Err(err) => return Result::Err(err)
            },
            Token::Let => match handle_let_token(tokens, &mut out, variable_scope_map, scope) {
                Result::Ok(_) => (),
                Result::Err(err) => return Result::Err(err)
            },
            Token::CurlyBracketOpen => {
                let exp: Exp = match parse_tokens(
                    tokens,
                    scope,
                    variable_scope_map,
                    vec![Token::CurlyBracketClosed],
                    Option::None
                ) {
                    Result::Ok((exp, _)) => exp,
                    Result::Err(err) => return Result::Err(err)
                };
                out.push(exp);
                // its ok to omit seq (";") after curly brackets. We place it for you.
                push_seq_if_not_present(tokens)
            }
            Token::CurlyBracketClosed => {
                match stop_on_scope {
                    Option::Some(s) if s < scope => break,
                    _ => ()
                }
            }
            Token::Else => return Result::Err(SyntaxError{})
        }
    }

    match final_process(&mut stack, &mut out) {
        Result::Ok(exp) => Result::Ok((exp, token_opt)),
        Result::Err(err) => Result::Err(err)
    }
}

fn final_process(stack: &mut Vec<Token>, out: &mut Vec<Exp>) -> Result<Exp, SyntaxError> {
    loop {
        match stack.pop() {
            Option::None => break,
            Option::Some(Token::RoundBracketOpen) => return Result::Err(SyntaxError{}),
            Option::Some(Token::Operator(op)) => {
                let result: Result<(), SyntaxError> = push_operator_to_out(&op, out);
                match result {
                    Result::Ok(()) => (),
                    Result::Err(err) => return Result::Err(err)
                };
            }
            Option::Some(Token::CurlyBracketOpen | Token::CurlyBracketClosed) => return Result::Err(SyntaxError{}),
            Option::Some(Token::RoundBracketClosed | Token::Operand(_) | Token::Let | Token::If | Token::Else) => panic!("This can never happer")
        }
    }
    
    if out.len() != 1 {
        Result::Err(SyntaxError{})
    } else {
        Result::Ok(out.pop().unwrap())
    }
}

fn handle_if_token(
    tokens: &mut Vec<Token>,
    out: &mut Vec<Exp>,
    variable_scope_map: &mut HashMap<String, usize>,
    scope: usize,
    stop_on_scope: Option<usize>
) -> Result<(), SyntaxError> {

    let condition: Exp = match parse_tokens(
        tokens,
        scope,
        variable_scope_map,
        vec![Token::CurlyBracketOpen],
        Option::None
    ) {
        Result::Ok((exp, _)) => exp,
        Result::Err(e) => return Result::Err(e)
    };

    let if_branch: Exp = match parse_tokens(
        tokens,
        scope,
        variable_scope_map,
        vec![Token::CurlyBracketClosed],
        Option::None
    ) {
        Result::Ok((exp, _)) => exp,
        Result::Err(e) => return Result::Err(e)
    };

    let else_branch: Exp = match tokens.last() {
        Option::Some(Token::Else) => {
            tokens.pop();
            match tokens.pop() {
                Option::Some(Token::CurlyBracketOpen) => match parse_tokens(
                    tokens,
                    scope,
                    variable_scope_map,
                    vec![Token::CurlyBracketClosed],
                    stop_on_scope
                ) {
                    Result::Ok((exp, _)) => exp,
                    Result::Err(err) => return Result::Err(err)
                }
                _ => return Result::Err(SyntaxError{})
            }
        }
        _ => Exp::Const(Const::None)
    };

    out.push(Exp::IfThenElse(Box::new(condition), Box::new(if_branch), Box::new(else_branch)));
    // its ok to omit seq (";") after curly brackets. We place it for you.
    push_seq_if_not_present(tokens);
    Result::Ok(())
}

fn handle_let_token(
    tokens: &mut Vec<Token>,
    out: &mut Vec<Exp>,
    variable_scope_map: &mut HashMap<String, usize>,
    scope: usize
) -> Result<(), SyntaxError> {

    // Parse variable after the let token
    let var: Var = match tokens.pop() {
        Option::Some(Token::Operand(Operand::Var(name))) => Var{name: name, scope: scope},
        _ => return Result::Err(SyntaxError{})
    };

    let value: Exp = match tokens.pop() {
        // If next token is '=' the variable is assigned during declaration
        Option::Some(Token::Operator(Operator::Assign)) => {
            match parse_tokens(
                tokens,
                scope,
                variable_scope_map,
                vec![Token::Operator(Operator::Seq)],
                Option::None
            ) {
                Result::Ok((exp, _)) => exp,
                Result::Err(err) => return Result::Err(err)
            }
        },
        // If next token is ';' we assign None to the variable during declaration
        Option::Some(Token::Operator(Operator::Seq)) => Exp::Const(Const::None),
        _ => return Result::Err(SyntaxError {})
    };

    // Store an eventually shadowed var
    let shadowed_var: Option<usize> = variable_scope_map.get(&var.name).map(|scop| { *scop });

    // Push this variable to the scope
    variable_scope_map.insert(var.name.clone(), scope);

    // Parse until this scope closure
    let exp: Exp = match parse_tokens(
        tokens,
        scope + 1,
        variable_scope_map,
        vec![],
        Option::Some(scope)
    ) {
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

    // Pop variable from scope and eventually restore shadowed variable
    match shadowed_var {
        Option::Some(scop) => variable_scope_map.insert(var.name.clone(), scop),
        Option::None => variable_scope_map.remove(&var.name)
    };

    // Push expression to the output queue
    out.push(Exp::Decl(var, Box::new(value), Box::new(exp)));
    Result::Ok(())
}

fn handle_round_bracket_closed_token(stack: &mut Vec<Token>, out: &mut Vec<Exp>) -> Result<(), SyntaxError> {
    loop {
        match stack.pop() {
            Option::None => return Result::Err(SyntaxError{}),
            Option::Some(Token::RoundBracketOpen) => break,
            Option::Some(Token::Operator(op)) => {
                let result: Result<(), SyntaxError> = push_operator_to_out(&op, out);
                match result {
                    Result::Ok(()) => (),
                    Result::Err(err) => return Result::Err(err)
                };
            },
            Option::Some(Token::CurlyBracketOpen | Token::CurlyBracketClosed) => return Result::Err(SyntaxError{}),
            Option::Some(Token::RoundBracketClosed | Token::Operand(_) | Token::Let | Token::If | Token::Else) => panic!("This can never happer")
        }
    };
    Result::Ok(())
}

fn handle_operator_token(op: &Operator, stack: &mut Vec<Token>, out: &mut Vec<Exp>) -> Result<(), SyntaxError> {
    loop {
        match stack.last() {
            Option::None => break,
            Option::Some(Token::RoundBracketOpen) => break,
            Option::Some(Token::Operator(o2)) => {
                if o2.precedence() > op.precedence() {
                    break;
                } else {
                    let result: Result<(), SyntaxError> = push_operator_to_out(o2, out);
                    match result {
                        Result::Ok(()) => stack.pop(),
                        Result::Err(err) => return Result::Err(err)
                    };
                }
            },
            Option::Some(Token::CurlyBracketOpen | Token::CurlyBracketClosed) => return Result::Err(SyntaxError{}),
            Option::Some(Token::RoundBracketClosed | Token::Operand(_) | Token::Let | Token::If | Token::Else) => panic!("This can never happer")
        }
    }
    stack.push(Token::Operator(*op));
    Result::Ok(())
}

fn push_operator_to_out(op: &Operator, out: &mut Vec<Exp>) -> Result<(), SyntaxError> {
    match op {
        Operator::Seq => {
            if out.is_empty() { return Result::Err(SyntaxError{}) }
            let exp2: Exp = out.pop().unwrap();
            let exp1: Exp = out.pop().unwrap_or(Exp::Const(Const::None));
            out.push(Exp::Seq(Box::new(exp1), Box::new(exp2)))
        },
        Operator::Assign => {
            if out.len() < 2 {
                return Result::Err(SyntaxError{})
            }
            let (exp, var) = (out.pop().unwrap(), out.pop().unwrap());
            match var {
                Exp::Var(var) => out.push(Exp::Assign(var, Box::new(exp))),
                _ => return Result::Err(SyntaxError{})
            }
        },
        Operator::Mul => {
            if out.len() < 2 {
                return Result::Err(SyntaxError{})
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Mul(Box::new(o1), Box::new(o2)))
        },
        Operator::Div => {
            if out.len() < 2 {
                return Result::Err(SyntaxError{})
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Div(Box::new(o1), Box::new(o2)))
        },
        Operator::Sum => {
            if out.len() < 2 {
                return Result::Err(SyntaxError{})
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Sum(Box::new(o1), Box::new(o2)))
        },
        Operator::Sub => {
            if out.len() < 2 {
                return Result::Err(SyntaxError{})
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Sub(Box::new(o1), Box::new(o2)))
        },
        Operator::Lt => {
            if out.len() < 2 {
                return Result::Err(SyntaxError{})
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Lt(Box::new(o1), Box::new(o2)))
        },
        Operator::Gt => {
            if out.len() < 2 {
                return Result::Err(SyntaxError{})
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Gt(Box::new(o1), Box::new(o2)))
        },
        Operator::Eq => {
            if out.len() < 2 {
                return Result::Err(SyntaxError{})
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Eq(Box::new(o1), Box::new(o2)))
        },
        Operator::And => {
            if out.len() < 2 {
                return Result::Err(SyntaxError{})
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::And(Box::new(o1), Box::new(o2)))
        },
        Operator::Or => {
            if out.len() < 2 {
                return Result::Err(SyntaxError{})
            }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Or(Box::new(o1), Box::new(o2)))
        },
        Operator::Not => {
            if out.len() < 1 {
                return Result::Err(SyntaxError{})
            }
            let o = out.pop().unwrap();
            out.push(Exp::Not(Box::new(o)))
        },
    }
    Result::Ok(())
}

/**
 *
 */
fn push_seq_if_not_present(tokens: &mut Vec<Token>) {
    match tokens.last() {
        Option::Some(Token::Operator(Operator::Seq)) | Option::None => (),
        _ => tokens.push(Token::Operator(Operator::Seq))
    }
}
