use std::collections::HashMap;
use std::io;
use std::io::Write;

mod expression;
mod token;
mod parser;
mod tokenizer;
mod semantics;

use tokenizer::tokenize;
use parser::parse;
use parser::parse_in_scope;
use semantics::eval;
use semantics::Error;

use token::Token;

use token::Operand;
use token::Operator;

use expression::Const;
use expression::Exp;

use crate::semantics::eval_expression;

fn main() {
    let mut stack: Vec<Const> = Vec::new();
    let mut variable_scope_map: HashMap<String, usize> = HashMap::new();
    let mut scope = 0;
    loop {
        // Reqest user input
        print!("epilang> ");
        let mut user_input = String::new();
        let stdin = io::stdin();
        io::stdout().flush().expect("flush failed!");
        stdin.read_line(&mut user_input).expect("failed to read stdin");
        // Check exit condition
        if user_input.trim() == "exit" {
            break
        }
        // Tokenize string
        let mut tokens: Vec<Token> = match tokenize(user_input) {
            Result::Ok(tokens) => tokens,
            Result::Err(err) => {
                println!("Syntax error during token parsing");
                continue
            }
        };
        // We need to handle let expression separately when in interactive mode
        let is_let: bool = match tokens.last() {
            Option::Some(Token::Let) => true,
            _ => false
        };
        if is_let {
            match eval_let(&mut tokens, &mut stack, scope, &mut variable_scope_map) {
                // Increment scope after evaluating let
                Result::Ok(new_scope) => scope = new_scope,
                Result::Err(msg) => println!("{}", msg)
            }
            continue;
        }
        // Parse tokens to exp
        let exp: Exp = match parse_in_scope(&mut tokens, scope, &mut variable_scope_map) {
            Result::Ok(exp) => exp,
            Result::Err(_) => {
                println!("Syntax error");
                continue
            }
        };
        // Evaluate expression
        match eval_expression(exp, &mut stack) {
            Result::Ok(Const::None) => (),
            Result::Ok(val) => println!("{}", val),
            Result::Err(err) => println!("Error: {}", err.msg)
        }
    }
}

/**
 * We need to handle let expression in a different way when
 * in interactive mode
 */
fn eval_let(
    tokens: &mut Vec<Token>,
    stack: &mut Vec<Const>,
    scope: usize,
    variable_scope_map: &mut HashMap<String, usize>
) -> Result<usize, String> {
    // Pop let token
    match tokens.pop() {
        Option::Some(Token::Let) => (),
        _ => panic!("First token must be a let when calling handle_let function")
    };
    // Pop variable token
    let var_name = match tokens.pop() {
        Option::Some(Token::Operand(Operand::Var(name))) => name,
        _ => return Result::Err(String::from("SyntaxError: expected variable token"))
    };

    // Pop "=" token
    match tokens.pop() {
        Option::Some(Token::Operator(Operator::Assign)) => (),
        _ => return Result::Err(String::from("SyntaxError: expected '=' token"))
    };

    let exp: Exp = match parse_in_scope(tokens, scope, variable_scope_map) {
        Result::Ok(exp) => exp,
        Result::Err(err) => return Result::Err(String::from("SyntaxError"))
    };
    let val = match eval_expression(exp, stack) {
        Result::Ok(val) => val,
        Result::Err(err) => return Result::Err(err.msg)
    };
    variable_scope_map.insert(var_name, scope);
    stack.push(val);
    Result::Ok(scope + 1)
}

fn const_to_string(c: &Const) -> String {
    match c {
        Const::Integer(i) => i.to_string(),
        Const::Boolean(b) => b.to_string(),
        Const::String(s) => format!("\"{}\"", s),
        Const::None => String::from("None")
    }
}

fn exp_to_string(exp: &Exp) -> String {
    match exp {
        Exp::Const(c) => const_to_string(c),
        Exp::Var(x) => x.scope.to_string(),
        Exp::Decl(x, val, e) => format!("let x{} = {};\n{}", x.scope, exp_to_string(val), exp_to_string(e)),
        Exp::Assign(x, e) => format!("x{} = {}", x.scope, exp_to_string(e)),
        Exp::Seq(e1, e2) => format!("{};\n {}", exp_to_string(e1), exp_to_string(e2)),
        Exp::Sum(e1, e2) => format!("{} + {}", exp_to_string(e1), exp_to_string(e2)),
        Exp::Sub(e1, e2) => format!("{} - {}", exp_to_string(e1), exp_to_string(e2)),
        Exp::Mul(e1, e2) => format!("{} * {}", exp_to_string(e1), exp_to_string(e2)),
        Exp::Div(e1, e2) => format!("{} / {}", exp_to_string(e1), exp_to_string(e2)),
        Exp::Lt(e1, e2) => format!("{} < {}", exp_to_string(e1), exp_to_string(e2)),
        Exp::Gt(e1, e2) => format!("{} > {}", exp_to_string(e1), exp_to_string(e2)),
        Exp::Eq(e1, e2) => format!("{} == {}", exp_to_string(e1), exp_to_string(e2)),
        Exp::And(e1, e2) => format!("{} && {}", exp_to_string(e1), exp_to_string(e2)),
        Exp::Or(e1, e2) => format!("{} || {}", exp_to_string(e1), exp_to_string(e2)),
        Exp::Not(e) => format!("!{}", exp_to_string(e)),
        Exp::IfThenElse(e, e1, e2) => format!("if {} {{ {} }} else {{ {} }}", exp_to_string(e), exp_to_string(e1), exp_to_string(e2))
    }
}


