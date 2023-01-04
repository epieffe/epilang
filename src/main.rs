mod expression;
mod token;
mod parser;
mod lexer;
mod semantics;
mod shell;
mod value;

use std::ops::Add;

use parser::parse;

use lexer::tokenize;

use token::Token;

use expression::{Const, Var};
use expression::Exp;

use semantics::eval;
use value::{Value};

fn main() {
    let mode = "SHELL";

    if mode == "SHELLaq" {
        shell::run_shell()
    } else {
        run_text()
    }
}

fn run_text() {
    let mut stack: Vec<Const> = Vec::new();
    let scope: usize = 0;

    let text = "let x = 3 ; let y = 4 ; let z = x + y + 2 ; z + x";

    let text = "let f = fn ( x , y ) { x + y } ; f ( 3 , 2 ) ; f ( 2 , 3 )";

    //let text = "let f = fn ( x , y ) { x + y } ; f ( 1 , f ( 5 , 6 ) )";

    //let text = "let x = 0 ; let y = 0 ; if ( x == 0 ) { y = 1 } else { y = 2 } ; y";

    // let text = String::from("if true {
    //     let x = 3 ;
    //     {
    //         let y = 3 ;
    //         y = 4 ;
    //         x ( y , 3 , y ( 4 ) , 4 + 4 )
    //     }
    //     4 ;
    //     x + 3 ;
    //     ! ! ( x == 3 )
    // } else { 4 }");

    let text = String::from(text);
    let mut tokens: Vec<Token> = tokenize(text).unwrap_or_else(|err| {
        panic!("TokenizerError")
    });

    // Parse tokens to exp
    let exp: Exp = parse(&mut tokens).unwrap_or_else(|err| {
        panic!("ParserError: {}", err.msg)
    });

    println!("{}", exp_to_string(&exp));
    println!("########");

    // Evaluate expression
    let val: Value = eval(exp).unwrap_or_else(|err| {
        panic!("RuntimeError: {}", err.msg)
    });

    println!("Result: {}", val);

}

fn const_to_string(c: &Const) -> String {
    match c {
        Const::Integer(i) => i.to_string(),
        Const::Boolean(b) => b.to_string(),
        Const::String(s) => format!("\"{}\"", s),
        Const::None => String::from("None")
    }
}

fn var_to_string(var: &Var) -> String {
    format!("{}_{}", var.name, var.scope)
}

fn vars_to_string(vars: &Vec<Var>) -> String {
    let names: Vec<String> = vars.iter().map(|var| {var_to_string(var)}).collect();
    format!("{}", names.join(", "))
}

pub fn exp_to_string(exp: &Exp) -> String {
    match exp {
        Exp::Const(c) => const_to_string(c),
        Exp::Var(x) => var_to_string(x),
        Exp::Decl(x, val, scope) => format!("let {} = {};\n{}", var_to_string(x), exp_to_string(val), exp_to_string(scope)),
        Exp::Function(args, body) => format!("fn ({}){{\n{}\n}}", vars_to_string(args), exp_to_string(body)),
        Exp::Assign(x, e) => format!("{} = {}", var_to_string(x), exp_to_string(e)),
        Exp::Seq(e1, e2) => format!("{};\n{}", exp_to_string(e1), exp_to_string(e2)),
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
        Exp::IfThenElse(e, e1, e2) => format!("if {} {{ {} }} else {{ {} }}", exp_to_string(e), exp_to_string(e1), exp_to_string(e2)),
        Exp::FunctionCall(e, args) => format!("{}({})", exp_to_string(e), args_to_string(args))
    }
}

fn args_to_string(args: &Vec<Exp>) -> String {
    args.iter().map(|exp| {exp_to_string(exp)}).reduce(|mut a, b| {
        a.push_str(&format!(", {}", b));
        return a;
    })
    .unwrap_or(String::from(" "))
}


// ###################################### TEST ###########################################################

#[cfg(test)]
mod tests {
    use parser::parse_tokens;
    use std::collections::HashMap;
    use crate::expression::Var;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn eval_program(text: String) -> Result<Const, ()> {
        let mut stack: Vec<Const> = Vec::new();
        let mut variable_scope_map: HashMap<String, Var> = HashMap::new();
        let scope: usize = 0;
    
        let mut tokens: Vec<Token> = tokenize(text)?;
    
        // Parse tokens to exp
        let exp: Exp = parse_tokens(&mut tokens, scope, &mut variable_scope_map).or(Result::Err(()))?;
    
        // Evaluate expression
        let val: Const = eval_expression(exp, &mut stack).or(Result::Err(()))?;
    
        Result::Ok(val)
    }

    #[test]
    fn test1() {
        assert_eq!(eval_program(String::from("2 + 2")), Result::Ok(Const::Integer(4)));
        assert_eq!(eval_program(String::from("2 - 2")), Result::Ok(Const::Integer(0)));
        assert_eq!(eval_program(String::from("2 * 2")), Result::Ok(Const::Integer(4)));
        assert_eq!(eval_program(String::from("2 / 2")), Result::Ok(Const::Integer(1)));
        assert_eq!(eval_program(String::from("2 + 2 * 3")), Result::Ok(Const::Integer(8)));
        assert_eq!(eval_program(String::from("2 - 2 * 3")), Result::Ok(Const::Integer(-4)));
        assert_eq!(eval_program(String::from("2 * 2 + 3 * 3")), Result::Ok(Const::Integer(13)));
        assert_eq!(eval_program(String::from("2 / 1 + 1")), Result::Ok(Const::Integer(3)));
    }

    #[test]
    fn test2() {
        assert_eq!(eval_program(String::from("2 < 3")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("2 > 3")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("2 + 2 < 3")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("2 + 2 > 3")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("10 * 10 > 90 + 10 - 1 ")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("10 * 10 < 90 + 10 - 1 ")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("100 / 10 > 3 + 3 ")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("100 / 10 < 3 + 3 ")), Result::Ok(Const::Boolean(false)));
    }

    #[test]
    fn test3() {
        assert_eq!(eval_program(String::from("true")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("false")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("! true")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("! false")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("true && true")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("true && ! true")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("true || true")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("true || ! true")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("! true || ! true")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("! ! true")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("! ! ! true")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("! ! false")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("! ! ! false")), Result::Ok(Const::Boolean(true)));
    }

    #[test]
    fn test4() {
        assert_eq!(eval_program(String::from("true == true")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("true == false")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("1 == 1")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("1 == 2")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("2 * 2 == 2 + 2 / 2 + 1")), Result::Ok(Const::Boolean(true)));
    }

    #[test]
    fn test5() {
        assert_eq!(eval_program(String::from("true != true")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("true != false")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("1 != 1")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("1 != 2")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("2 * 2 != 2 + 2 / 2 + 1")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("! 3 == 3")), Result::Ok(Const::Boolean(false)));
        assert_eq!(eval_program(String::from("( ! 3 == 3 ) == ( 3 != 3 )")), Result::Ok(Const::Boolean(true)));
        assert_eq!(eval_program(String::from("( ! 3 != 3 ) == ( 3 == 3 )")), Result::Ok(Const::Boolean(true)));
    }

    #[test]
    fn test6() {
        assert_eq!(eval_program(String::from("let x = 0 ; let y = 0 ; if (x == 0) { y = 1 } else { y = 2 } ; y")), Result::Ok(Const::Integer(1)));
        assert_eq!(eval_program(String::from("let x = 1 ; let y = 0 ; if (x == 0) { y = 1 } else { y = 2 } ; y")), Result::Ok(Const::Integer(2)));
        
        let text1 = String::from("
            let x = 0 ;
            let y = 0 ;
            if (x == 0) {
                if (y == 0) {
                    0 ; 
                }
                else {
                    1 ;
                }
            }
            else {
                2 ;
            }");
        
        assert_eq!(eval_program(text1), Result::Ok(Const::Integer(0)));

        let text2 = String::from("
            let x = 0 ;
            let y = 1 ;
            if (x == 0) {
                if (y == 0) {
                    0 ; 
                }
                else {
                    1 ;
                }
            }
            else {
                2 ;
            }");
        
        assert_eq!(eval_program(text2), Result::Ok(Const::Integer(1)));

        let text3 = String::from("
            let x = 1 ;
            let y = 0 ;
            if (x == 0) {
                if (y == 0) {
                    0 ; 
                }
                else {
                    1 ;
                }
            }
            else {
                2 ;
            }");
        
        assert_eq!(eval_program(text3), Result::Ok(Const::Integer(2)));
    }

}


