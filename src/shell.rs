use std::collections::HashMap;

use rustyline::error::ReadlineError;
use rustyline::{Editor};

use crate::lexer::tokenize;
use crate::parser::parse_tokens;
use crate::semantics::eval_expression;
use crate::value::{Value, StackValue};

use crate::expression::Exp;
use crate::expression::Const;
use crate::expression::Var;

use crate::token::Token;
use crate::token::Operand;
use crate::token::Operator;

pub fn run_shell() {
    let mut stack: Vec<StackValue> = Vec::new();
    let mut variable_scope_map: HashMap<String, usize> = HashMap::new();
    let scope = 0;

    let mut rl: Editor<()> = Editor::<()>::new().expect("Error creating editor");
    loop {
        match rl.readline("epilang> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                handle_user_input(line, &mut stack, scope, &mut variable_scope_map)
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("ReadlineError: {:?}", err);
                break
            }
        };
    }
}

/**
 * Evaluating user input can change the actual scope.
 * The new scope is returned.
 */
fn handle_user_input(
    line: String,
    stack: &mut Vec<StackValue>,
    mut scope: usize,
    variable_scope_map: &mut HashMap<String, usize>
) -> usize {
    // Tokenize string
    let mut tokens: Vec<Token> = match tokenize(line) {
        Result::Ok(tokens) => tokens,
        Result::Err(err) => {
            println!("Syntax error during token parsing");
            return scope
        }
    };
    if tokens.is_empty() { return scope }

    // We need to handle let expression separately when in interactive mode
    let is_let: bool = match tokens.last() {
        Option::Some(Token::Let) => true,
        _ => false
    };
    if is_let {
        match eval_let(&mut tokens, stack, scope, variable_scope_map) {
            // Increment scope after evaluating let
            Result::Ok(new_scope) => scope = new_scope,
            Result::Err(msg) => println!("{}", msg)
        }
        return scope;
    }
    // Parse tokens to exp
    let exp: Exp = match parse_tokens(&mut tokens, scope, variable_scope_map) {
        Result::Ok(exp) => exp,
        Result::Err(_) => {
            println!("Syntax error");
            return scope
        }
    };
    // Evaluate expression
    match eval_expression(&exp, stack) {
        Result::Ok(Value::Unit) => (),
        Result::Ok(val) => {
            println!("{}", val);
        },
        Result::Err(err) => println!("Error: {}", err.msg)
    };
    scope
}

/**
 * We need to handle let expression in a different way when
 * in interactive mode
 */
fn eval_let(
    tokens: &mut Vec<Token>,
    stack: &mut Vec<StackValue>,
    scope: usize,
    variable_scope_map: &mut HashMap<String, usize>
) -> Result<usize, String> {
    // Pop let token
    match tokens.pop() {
        Option::Some(Token::Let) => (),
        _ => panic!("First token must be a let when calling handle_let function")
    };
    // Pop variable token
    let var_name: String = match tokens.pop() {
        Option::Some(Token::Operand(Operand::Var(name))) => name,
        _ => return Result::Err(String::from("SyntaxError: expected variable token"))
    };

    // Pop "=" token
    match tokens.pop() {
        Option::Some(Token::Operator(Operator::Assign)) => (),
        _ => return Result::Err(String::from("SyntaxError: expected '=' token"))
    };

    let exp: Exp = match parse_tokens(tokens, scope + 1, variable_scope_map) {
        Result::Ok(exp) => exp,
        Result::Err(err) => return Result::Err(String::from(format!("SyntaxError: {}", err.msg)))
    };
    let val = match eval_expression(&exp, stack) {
        Result::Ok(val) => val,
        Result::Err(err) => return Result::Err(err.msg)
    };
    variable_scope_map.insert(var_name, scope);
    stack.push(StackValue::from_box(Box::new(val)));
    Result::Ok(scope + 1)
}
