mod expression;
mod token;
mod parser;
mod tokenizer;
mod semantics;

use tokenizer::tokenize;
use parser::parse;
use semantics::eval;

use token::Token;

use expression::Const;
use expression::Exp;

fn main() {
    let p = "
    if true {
        let x = 3 ;
        {
            let y = 3 ;
            y = 4
        }
        4 ;
        y + 3
    } else { 4 }";
    let mut tokens: Vec<Token> = tokenize(String::from(p))
        .expect("Errore tokens");

    let exp = match parse(&mut tokens) {
        Result::Ok(exp) => {
            println!("{}", exp_to_string(&exp));
            exp
        },
        Result::Err(_) => panic!("Syntax error")
    };

    println!("######");
    let val = match eval(exp) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("Error: {}", err.msg)
    };
    println!("Result: {}", val)
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


