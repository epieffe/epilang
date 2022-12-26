mod expression;
mod token;
mod parser;
mod tokenizer;

use tokenizer::tokenize;
use parser::parse;

use token::Token;
use token::Operand;
use token::Operator;

use expression::Var;
use expression::Const;
use expression::Const::Integer;
use expression::Exp;

fn main() {
    /*
    let x = Var{name: String::from("x")};

    let c = Exp::Const(Integer(3));
    let ass = Exp::Assign(x, Box::new(c));

    let p = Exp::Decl(Var{name: String::from("x")}, Box::new(ass));

    let mut tokens = vec![
        Token::Operand(Operand::Var(String::from("x"))),
        Token::Operator(Operator::Eq),
        Token::Operand(Operand::Int(1)),
        Token::Operator(Operator::Seq),
        Token::Operand(Operand::Var(String::from("y"))),
        Token::Operator(Operator::Eq),
        Token::Operand(Operand::Int(2)),
    ];
    */
    let p = "if true { { let y = 3 ; y = 4 } ; 4 } else { 4 } ; { let x ; x = 4 }";
    let mut tokens: Vec<Token> = tokenize(String::from(p))
        .expect("Errore tokens");

    match parse(&mut tokens) {
        Result::Err(_) => panic!("Syntex error"),
        Result::Ok(exp) => {
            println!("{}", exp_to_string(exp));
        }
    }
}

fn const_to_string(c: Const) -> String {
    match c {
        Const::Integer(i) => i.to_string(),
        Const::Boolean(b) => b.to_string(),
        Const::String(s) => format!("\"{}\"", s),
        Const::None => String::from("None")
    }
}

fn exp_to_string(exp: Exp) -> String {
    match exp {
        Exp::Const(c) => const_to_string(c),
        Exp::Var(x) => x.name,
        Exp::Decl(x, val, e) => format!("let {} = {};\n{}", x.name, exp_to_string(*val), exp_to_string(*e)),
        Exp::Assign(x, e) => format!("{} = {}", x.name, exp_to_string(*e)),
        Exp::Seq(e1, e2) => format!("{};\n {}", exp_to_string(*e1), exp_to_string(*e2)),
        Exp::Sum(e1, e2) => format!("{} + {}", exp_to_string(*e1), exp_to_string(*e2)),
        Exp::Sub(e1, e2) => format!("{} - {}", exp_to_string(*e1), exp_to_string(*e2)),
        Exp::Mul(e1, e2) => format!("{} * {}", exp_to_string(*e1), exp_to_string(*e2)),
        Exp::Div(e1, e2) => format!("{} / {}", exp_to_string(*e1), exp_to_string(*e2)),
        Exp::Lt(e1, e2) => format!("{} < {}", exp_to_string(*e1), exp_to_string(*e2)),
        Exp::Gt(e1, e2) => format!("{} > {}", exp_to_string(*e1), exp_to_string(*e2)),
        Exp::Eq(e1, e2) => format!("{} == {}", exp_to_string(*e1), exp_to_string(*e2)),
        Exp::And(e1, e2) => format!("{} && {}", exp_to_string(*e1), exp_to_string(*e2)),
        Exp::Or(e1, e2) => format!("{} || {}", exp_to_string(*e1), exp_to_string(*e2)),
        Exp::Not(e) => format!("!{}", exp_to_string(*e)),
        Exp::IfThenElse(e, e1, e2) => format!("if {} {{ {} }} else {{ {} }}", exp_to_string(*e), exp_to_string(*e1), exp_to_string(*e2))
    }
}


