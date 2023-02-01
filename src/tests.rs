#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use crate::semantics::eval;
    use crate::value::{V, Value};
    use crate::expression::Exp;
    use crate::token::Token;
    use crate::lexer::tokenize;


    fn eval_program(text: String) -> Result<V, ()> {
        let mut tokens: Vec<Token> = tokenize(text).or(Result::Err(()))?;

        // Parse tokens to exp
        let exp: Exp = parse(&mut tokens).or(Result::Err(()))?;

        // Evaluate expression
        let val = eval(&exp).or(Result::Err(()))?;
        Result::Ok(val)
    }

    #[test]
    fn test1() {
        assert_eq!(eval_program(String::from("2 + 2")), Result::Ok(V::Val(Value::Int(4))));
        assert_eq!(eval_program(String::from("2 - 2")), Result::Ok(V::Val(Value::Int(0))));
        assert_eq!(eval_program(String::from("2 * 2")), Result::Ok(V::Val(Value::Int(4))));
        assert_eq!(eval_program(String::from("2 / 2")), Result::Ok(V::Val(Value::Int(1))));
        assert_eq!(eval_program(String::from("2 + 2 * 3")), Result::Ok(V::Val(Value::Int(8))));
        assert_eq!(eval_program(String::from("2 - 2 * 3")), Result::Ok(V::Val(Value::Int(-4))));
        assert_eq!(eval_program(String::from("2 * 2 + 3 * 3")), Result::Ok(V::Val(Value::Int(13))));
        assert_eq!(eval_program(String::from("2 / 1 + 1")), Result::Ok(V::Val(Value::Int(3))));
    }

    #[test]
    fn test2() {
        assert_eq!(eval_program(String::from("2 < 3")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("2 > 3")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("2 + 2 < 3")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("2 + 2 > 3")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("10 * 10 > 90 + 10 - 1 ")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("10 * 10 < 90 + 10 - 1 ")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("100 / 10 > 3 + 3 ")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("100 / 10 < 3 + 3 ")), Result::Ok(V::Val(Value::Bool(false))));
    }

    #[test]
    fn test3() {
        assert_eq!(eval_program(String::from("true")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("false")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("! true")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("! false")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("true && true")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("true && ! true")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("true || true")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("true || ! true")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("! true || ! true")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("! ! true")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("! ! ! true")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("! ! false")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("! ! ! false")), Result::Ok(V::Val(Value::Bool(true))));
    }

    #[test]
    fn test4() {
        assert_eq!(eval_program(String::from("true == true")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("true == false")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("1 == 1")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("1 == 2")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("2 * 2 == 2 + 2 / 2 + 1")), Result::Ok(V::Val(Value::Bool(true))));
    }

    #[test]
    fn test5() {
        assert_eq!(eval_program(String::from("true != true")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("true != false")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("1 != 1")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("1 != 2")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("2 * 2 != 2 + 2 / 2 + 1")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("! 3 == 3")), Result::Ok(V::Val(Value::Bool(false))));
        assert_eq!(eval_program(String::from("( ! 3 == 3 ) == ( 3 != 3 )")), Result::Ok(V::Val(Value::Bool(true))));
        assert_eq!(eval_program(String::from("( ! 3 != 3 ) == ( 3 == 3 )")), Result::Ok(V::Val(Value::Bool(true))));
    }

    #[test]
    fn test6() {
        assert_eq!(eval_program(String::from("let x = 0 ; let y = 0 ; if (x == 0) { y = 1 } else { y = 2 } ; y")), Result::Ok(V::Val(Value::Int(1))));
        assert_eq!(eval_program(String::from("let x = 1 ; let y = 0 ; if (x == 0) { y = 1 } else { y = 2 } ; y")), Result::Ok(V::Val(Value::Int(2))));
        let text1 = String::from("
            let x = 0 ;
            let y = 0 ;
            if (x == 0) {
                if (y == 0) {
                    0
                }
                else {
                    1
                }
            }
            else {
                2
            }");
        assert_eq!(eval_program(text1), Result::Ok(V::Val(Value::Int(0))));

        let text2 = String::from("
            let x = 0 ;
            let y = 1 ;
            if (x == 0) {
                if (y == 0) {
                    0
                }
                else {
                    1
                }
            }
            else {
                2
            }");
        assert_eq!(eval_program(text2), Result::Ok(V::Val(Value::Int(1))));

        let text3 = String::from("
            let x = 1 ;
            let y = 0 ;
            if (x == 0) {
                if (y == 0) {
                    0
                }
                else {
                    1
                }
            }
            else {
                2
            }");
        assert_eq!(eval_program(text3), Result::Ok(V::Val(Value::Int(2))));
    }

}
