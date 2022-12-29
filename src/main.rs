mod expression;
mod token;
mod parser;
mod tokenizer;
mod semantics;
mod shell;

use std::collections::HashMap;

use tokenizer::tokenize;
use parser::parse_in_scope;

use token::Token;

use expression::Const;
use expression::Exp;

use crate::semantics::eval_expression;

fn main() {
    let mode = "SHELL";

    if mode == "SHELL" {
        shell::run_shell()
    } else {
        run_text()
    }
}

fn run_text() {
    let mut stack: Vec<Const> = Vec::new();
    let mut variable_scope_map: HashMap<String, usize> = HashMap::new();
    let scope: usize = 0;

    let text = String::from("if true {
        let x = 3 ;
        {
            let y = 3 ;
            y = 4
        }
        4 ;
        x + 3 ;
        ! ! ( x == 3 )
    } else { 4 }");
    let mut tokens: Vec<Token> = tokenize(text).unwrap_or_else(|err| {
        panic!("TokenizerError")
    });

    // Parse tokens to exp
    let exp: Exp = parse_in_scope(&mut tokens, scope, &mut variable_scope_map).unwrap_or_else(|err| {
        panic!("ParserError")
    });

    println!("{}", exp_to_string(&exp));
    println!("########");

    // Evaluate expression
    let val: Const = eval_expression(exp, &mut stack).unwrap_or_else(|err| {
        panic!("RuntimeError: {}", err.msg)
    });

    println!("Result: {}", const_to_string(&val));

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
        Exp::Var(x) => format!("x{}", x.scope.to_string()),
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
        Exp::Neq(e1, e2) => format!("{} != {}", exp_to_string(e1), exp_to_string(e2)),
        Exp::And(e1, e2) => format!("{} && {}", exp_to_string(e1), exp_to_string(e2)),
        Exp::Or(e1, e2) => format!("{} || {}", exp_to_string(e1), exp_to_string(e2)),
        Exp::Not(e) => format!("!{}", exp_to_string(e)),
        Exp::IfThenElse(e, e1, e2) => format!("if {} {{ {} }} else {{ {} }}", exp_to_string(e), exp_to_string(e1), exp_to_string(e2))
    }
}
