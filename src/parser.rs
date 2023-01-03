use std::collections::HashMap;

use crate::expression::Exp;
use crate::expression::Const;
use crate::expression::Var;

use crate::token::Token;
use crate::token::Operand;
use crate::token::Operator;

pub struct SyntaxError {
    pub msg: String
}

pub fn parse(tokens: &mut Vec<Token>) -> Result<Exp, SyntaxError> {
    let mut variable_map = HashMap::new();
    parse_tokens(tokens, 0, &mut variable_map)
}

pub fn parse_tokens(
    tokens: &mut Vec<Token>,
    scope: usize,
    variable_map: &mut HashMap<String, Var>,
) -> Result<Exp, SyntaxError> {

    let mut stack: Vec<Token> = Vec::new();
    let mut out: Vec<Exp> = Vec::new();

    let mut scope = scope;
    loop {
        match tokens.pop().unwrap_or(Token::EOF) {
            Token::EOF => break,

            // Push variable to out. Error if not present in scope
            Token::Operand(Operand::Var(x)) => match variable_map.get(&x) {
                Option::Some(v) => out.push(Exp::Var(Var{name: x, scope: v.scope})),
                Option::None => return Result::Err(SyntaxError{msg: String::from(format!("Unknow variable {}", x))})
            }
            // Operands that are not variables are easily pushed to out
            Token::Operand(o) => out.push(o.to_exp()),

            // Handle operator tokens. Remember that also `;` is an operator, like also `+`, `=`, etc...
            Token::Operator(op) => {
                match op {
                    Operator::Seq => match tokens.last() {
                        // If `;` is the last token in a scope we automatically add `unit` at the end
                        Option::Some(Token::CurlyBracketClosed) => tokens.push(Token::Operand(Operand::Null)),
                        Option::None => tokens.push(Token::Operand(Operand::Null)),
                        // Reduce sequences of multiple subsequent `;` to only one element
                        Option::Some(Token::Operator(Operator::Seq)) => continue,
                        _ => ()
                    },
                    _ => ()
                }
                handle_operator_token(op, &mut stack, &mut out)?
            },

            Token::RoundBracketOpen => stack.push(Token::RoundBracketOpen),

            Token::RoundBracketClosed => handle_round_bracket_closed_token(&mut stack, &mut out)?,

            Token::If => stack.push(Token::If),

            Token::Else => stack.push(Token::Else),

            Token::Let => {
                // Declaring a variable increments the scope
                scope = handle_let_token(tokens, &mut stack, variable_map, scope)?
            },

            Token::CurlyBracketOpen => stack.push(Token::CurlyBracketOpen),

            Token::CurlyBracketClosed => {
                // Closing curly brackets can decrement scope
                scope = handle_curly_bracket_closed_token(&mut stack, &mut out, scope)?;
                // After closing a curly bracket we automatically insert `;` if not present.
                // This makes the syntax more similar to Java, C++ etc
                match tokens.last() {
                    Option::Some(Token::Operator(Operator::Seq)) => (),
                    Option::Some(Token::Else) => (),
                    Option::None => (),
                    _ => tokens.push(Token::Operator(Operator::Seq))
                }
            },
        }
    }

    loop {
        match stack.pop() {
            Option::None => break,
            Option::Some(Token::RoundBracketOpen) => return Result::Err(SyntaxError{msg: String::from("Unexpected character (")}),
            Option::Some(Token::Operator(op)) => {
                let result: Result<(), SyntaxError> = push_operator_to_out(&op, &mut out);
                match result {
                    Result::Ok(()) => (),
                    Result::Err(err) => return Result::Err(err)
                };
            }
            Option::Some(Token::Let) => {
                scope -= 1;
                push_let_expr_to_out(&mut out, scope)?
            }
            Option::Some(Token::CurlyBracketOpen) => return Result::Err(SyntaxError{msg: String::from("Unexpected character {")}),
            Option::Some(Token::CurlyBracketClosed) => return Result::Err(SyntaxError{msg: String::from("Unexpected character }")}),
            Option::Some(Token::If) => return Result::Err(SyntaxError{msg: String::from("Unexpected if")}),
            Option::Some(Token::Else) => return Result::Err(SyntaxError{msg: String::from("Unexpected else")}),
            Option::Some(Token::RoundBracketClosed) => panic!("Found RoundBracketClosed in parser operator stack"),
            Option::Some(Token::Operand(_)) => panic!("Found Operand in parser operator stack"),
            Option::Some(Token::EOF) => panic!("Found EOF in parser operator stack")
        }
    }
    
    if out.len() != 1 {
        Result::Err(SyntaxError{msg: String::from("Can not parse a single expression. Probabily missing a ;")})
    } else if (!stack.is_empty()) {
        Result::Err(SyntaxError{msg: String::from("Can not parse a single expression. Probabily missing a ;")})
    } else {
        Result::Ok(out.pop().unwrap())
    }
}

fn handle_let_token(
    tokens: &mut Vec<Token>,
    stack: &mut Vec<Token>,
    variable_map: &mut HashMap<(String), Var>,
    scope: usize
) -> Result<usize, SyntaxError> {

    match tokens.last() {
        Option::Some(Token::Operand(Operand::Var(var))) => {
            variable_map.insert(var.clone(), Var{name: var.clone(), scope: scope});
        },
        _ => return Result::Err(SyntaxError{msg: String::from("Expected variable name after let")})
    };

    stack.push(Token::Let);
    Result::Ok(scope + 1)
}

/**
 * This function is called when a CurlyBracketClosed Token is found.
 * While popping elements from the operator stack, we decrement the scope by 1 every time we find
 * a Let token. The new scope value is returned.
 */
fn handle_curly_bracket_closed_token(stack: &mut Vec<Token>, out: &mut Vec<Exp>, mut scope: usize) -> Result<usize, SyntaxError> {
    loop {
        match stack.pop() {
            Option::Some(Token::CurlyBracketOpen) => break,
            Option::Some(Token::Operator(op)) => {
                let result: Result<(), SyntaxError> = push_operator_to_out(&op, out);
                match result {
                    Result::Ok(()) => (),
                    Result::Err(err) => return Result::Err(err)
                };
            },
            // If we find a let token we decrement the scope and build the let expression
            Option::Some(Token::Let) => {
                scope -= 1;
                push_let_expr_to_out(out, scope)?
            },
            Option::None => return Result::Err(SyntaxError{msg: String::from("Curly brackets mismatch")}),
            Option::Some(Token::RoundBracketOpen) => return Result::Err(SyntaxError{msg: String::from("Round brackets mismatch")}),
            Option::Some(Token::RoundBracketClosed) => panic!("Found RoundBracketClosed in parser operator stack"),
            Option::Some(Token::CurlyBracketClosed) => panic!("Found CurlyBracketClosed in parser operator stack"),
            Option::Some(Token::Operand(_)) => panic!("Found Operand in parser operator stack"),
            Option::Some(Token::If) => panic!("Found If in parser operator stack"),
            Option::Some(Token::Else) => panic!("Found Else in parser operator stack"),
            Option::Some(Token::EOF) => panic!("Found EOF in parser operator stack")
        }
    };
    match stack.last() {
        // Check if this curly bracket closes an if scope
        Option::Some(Token::If) => {
            stack.pop();
            if out.len() < 2 { return Result::Err(SyntaxError{msg: String::from("Malformed if")}) }
            let if_branch: Exp = out.pop().unwrap();
            let if_clause: Exp = out.pop().unwrap();
            out.push(Exp::IfThenElse(Box::new(if_clause), Box::new(if_branch), Box::new(Exp::Const(Const::None))))
        },
        // Check if this curly bracket closes an else scope
        Option::Some(Token::Else) => {
            stack.pop();
            if out.len() < 2 { return Result::Err(SyntaxError{msg: String::from("Malformed else")}) }
            let else_branch: Exp = out.pop().unwrap();
            match out.pop() {
                Option::Some(Exp::IfThenElse(clause, if_branch, none_branch)) => {
                    match *none_branch {
                        Exp::Const(Const::None) => {
                            out.push(Exp::IfThenElse(clause, if_branch, Box::new(else_branch)))
                        },
                        _ => return Result::Err(SyntaxError{msg: String::from("If expression already has an else branch")})
                    }
                },
                _ => return Result::Err(SyntaxError{msg: String::from("Unexpected else")})
            }
        },
        _ => ()
    };
    // Return new scope value
    Result::Ok(scope)
}

fn handle_round_bracket_closed_token(stack: &mut Vec<Token>, out: &mut Vec<Exp>) -> Result<(), SyntaxError> {
    loop {
        match stack.pop() {
            Option::None => return Result::Err(SyntaxError{msg: String::from("Mismatched round brackets")}),
            Option::Some(Token::RoundBracketOpen) => break,
            Option::Some(Token::Operator(op)) => {
                let result: Result<(), SyntaxError> = push_operator_to_out(&op, out);
                match result {
                    Result::Ok(()) => (),
                    Result::Err(err) => return Result::Err(err)
                };
            },
            // If we find a let token we decrement the scope and build the let expression
            Option::Some(Token::Let) => return Result::Err(SyntaxError{msg: String::from("Unexpected let statement in round brackets")}),
            Option::Some(Token::CurlyBracketOpen) => return Result::Err(SyntaxError{msg: String::from("Round brackets mismatch")}),
            Option::Some(Token::RoundBracketClosed) => panic!("Found RoundBracketClosed in parser operator stack"),
            Option::Some(Token::CurlyBracketClosed) => panic!("Found CurlyBracketClosed in parser operator stack"),
            Option::Some(Token::Operand(_)) => panic!("Found Operand in parser operator stack"),
            Option::Some(Token::If) => panic!("Found If in parser operator stack"),
            Option::Some(Token::Else) => panic!("Found Else in parser operator stack"),
            Option::Some(Token::EOF) => panic!("Found EOF in parser operator stack")
        }
    };
    Result::Ok(())
}

fn handle_operator_token(op: Operator, stack: &mut Vec<Token>, out: &mut Vec<Exp>) -> Result<(), SyntaxError> {
    loop {
        match stack.last() {
            Option::None => break,
            Option::Some(Token::RoundBracketOpen) => break,
            Option::Some(Token::CurlyBracketOpen) => break,
            Option::Some(Token::Let) => break,
            Option::Some(Token::If) => break,
            Option::Some(Token::Else) => break,
            Option::Some(Token::Operator(o2)) => {
                if o2.precedence() >= op.precedence() {
                    break
                } else {
                    match push_operator_to_out(o2, out) {
                        Result::Ok(()) => stack.pop(),
                        Result::Err(err) => return Result::Err(err)
                    };
                }
            },
            Option::Some(Token::CurlyBracketClosed) => panic!("Found CurlyBracketClosed in parser operator stack"),
            Option::Some(Token::RoundBracketClosed) => panic!("Found RoundBracketClosed in parser operator stack"),
            Option::Some(Token::Operand(_) ) => panic!("Found Operand in parser operator stack"),
            Option::Some(Token::EOF) => panic!("Found EOF in parser operator stack")
        }
    }
    stack.push(Token::Operator(op));
    Result::Ok(())
}

fn push_let_expr_to_out(out: &mut Vec<Exp>, scope: usize) -> Result<(), SyntaxError> {
    let (exp1, exp2) = match out.pop() {
        // If the next element in the queue is not `;` return error
        Option::Some(Exp::Seq(exp1, exp2)) => (*exp1, exp2),
        _ => return Result::Err(SyntaxError { msg:  String::from("Expected ; after let")})
    };
    match exp1 {
        // Case when variable is assigned during declaration
        Exp::Assign(var, right_exp) => {
            if var.scope != scope {
                return Result::Err(SyntaxError{
                    msg: String::from(format!("Variable {} has scope {}, but was expecting scope {}", var.name, var.scope, scope))
                })
            }
            out.push(Exp::Decl(var, right_exp, exp2));
        },
        // Case when variable is declared but not assigned
        Exp::Var(var) => {
            if var.scope != scope {
                return Result::Err(SyntaxError{
                    msg: String::from(format!("Variable {} has scope {}, but was expecting scope {}", var.name, var.scope, scope))
                })
            }
            let none = Box::new(Exp::Const(Const::None));
            out.push(Exp::Decl(var, none, exp2));
        },
        _ => return Result::Err(SyntaxError{msg: String::from("Expecting variable or assignment after let")})
    };
    Result::Ok(())
}

fn push_operator_to_out(op: &Operator, out: &mut Vec<Exp>) -> Result<(), SyntaxError> {
    match op {
        Operator::Seq => {
            if out.is_empty() { return Result::Err(SyntaxError{msg: format!("Unexpected operator {}", op)}) }
            let exp2: Exp = out.pop().unwrap();
            let exp1: Exp = out.pop().unwrap_or(Exp::Const(Const::None));
            out.push(Exp::Seq(Box::new(exp1), Box::new(exp2)))
        },
        Operator::Assign => {
            if out.len() < 2 { return Result::Err(SyntaxError{msg: format!("Unexpected operator {}", op)}) }
            let (exp, var) = (out.pop().unwrap(), out.pop().unwrap());
            match var {
                Exp::Var(var) => out.push(Exp::Assign(var, Box::new(exp))),
                _ => return Result::Err(SyntaxError{msg: String::from("Expected variable before assignment")})
            }
        },
        Operator::Mul => {
            if out.len() < 2 { return Result::Err(SyntaxError{msg: format!("Unexpected operator {}", op)}) }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Mul(Box::new(o1), Box::new(o2)))
        },
        Operator::Div => {
            if out.len() < 2 { return Result::Err(SyntaxError{msg: format!("Unexpected operator {}", op)}) }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Div(Box::new(o1), Box::new(o2)))
        },
        Operator::Sum => {
            if out.len() < 2 { return Result::Err(SyntaxError{msg: format!("Unexpected operator {}", op)}) }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Sum(Box::new(o1), Box::new(o2)))
        },
        Operator::Sub => {
            if out.len() < 2 { return Result::Err(SyntaxError{msg: format!("Unexpected operator {}", op)}) }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Sub(Box::new(o1), Box::new(o2)))
        },
        Operator::Lt => {
            if out.len() < 2 { return Result::Err(SyntaxError{msg: format!("Unexpected operator {}", op)}) }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Lt(Box::new(o1), Box::new(o2)))
        },
        Operator::Gt => {
            if out.len() < 2 { return Result::Err(SyntaxError{msg: format!("Unexpected operator {}", op)}) }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Gt(Box::new(o1), Box::new(o2)))
        },
        Operator::Eq => {
            if out.len() < 2 { return Result::Err(SyntaxError{msg: format!("Unexpected operator {}", op)}) }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Eq(Box::new(o1), Box::new(o2)))
        },
        Operator::Neq => {
            if out.len() < 2 { return Result::Err(SyntaxError{msg: format!("Unexpected operator {}", op)}) }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Neq(Box::new(o1), Box::new(o2)))
        },
        Operator::And => {
            if out.len() < 2 { return Result::Err(SyntaxError{msg: format!("Unexpected operator {}", op)}) }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::And(Box::new(o1), Box::new(o2)))
        },
        Operator::Or => {
            if out.len() < 2 { return Result::Err(SyntaxError{msg: format!("Unexpected operator {}", op)}) }
            let (o2, o1) = (out.pop().unwrap(), out.pop().unwrap());
            out.push(Exp::Or(Box::new(o1), Box::new(o2)))
        },
        Operator::Not => {
            if out.len() < 1 { return Result::Err(SyntaxError{msg: format!("Unexpected operator {}", op)}) }
            let o = out.pop().unwrap();
            out.push(Exp::Not(Box::new(o)))
        },
    }
    Result::Ok(())
}
