mod expression;
mod token;
mod parser;
mod lexer;
mod semantics;
mod shell;

use std::collections::HashMap;

use lexer::tokenize;
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







// ###################################### TEST ###########################################################


#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn eval_program(text: String) -> Result<Const, ()> {
        let mut stack: Vec<Const> = Vec::new();
        let mut variable_scope_map: HashMap<String, usize> = HashMap::new();
        let scope: usize = 0;
    
        let mut tokens: Vec<Token> = tokenize(text)?;
    
        // Parse tokens to exp
        let exp: Exp = parse_in_scope(&mut tokens, scope, &mut variable_scope_map).or(Result::Err(()))?;
    
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
        assert_eq!(eval_program(String::from("let x = 0 ; let y = 0 ; if x == 0 { y = 1 } else { y = 2 } ; y")), Result::Ok(Const::Integer(1)));
        assert_eq!(eval_program(String::from("let x = 1 ; let y = 0 ; if x == 0 { y = 1 } else { y = 2 } ; y")), Result::Ok(Const::Integer(2)));
        
        let text1 = String::from("
            let x = 0 ;
            let y = 0 ;
            if x == 0 {
                if y == 0 {
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
            if x == 0 {
                if y == 0 {
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
            if x == 0 {
                if y == 0 {
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


