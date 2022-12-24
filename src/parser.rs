use crate::token::Token;
use crate::expression::Exp;
use crate::expression::Operator;


pub fn parse(mut tokens: Vec<Token>) {
    let mut stack: Vec<Token> = Vec::new();
    let mut out: Vec<Exp> = Vec::new();

    loop {
        let token: Option<Token> = tokens.pop();
        match token {
            Option::None => break,
            Option::Some(Token::Operand(ref o)) => out.push(o.to_exp()),
            Option::Some(Token::RoundBracketOpen) => stack.push(Token::RoundBracketOpen),
            Option::Some(Token::CurlyBracketOpen) => stack.push(Token::CurlyBracketOpen),
            Option::Some(Token::Operator(ref o)) => {
                loop {
                    let t = stack.pop();
                    match t {
                        Option::None => break,
                        Option::Some(Token::RoundBracketOpen | Token::CurlyBracketOpen) => break,
                        Option::Some(Token::Operator(ref o2)) => {
                            if o2.precedence() > o.precedence() {
                                break;
                            } else {
                                match o2 {
                                    Operator::Assign => {
                                        
                                    },
                                    Operator::Mul => 1,
                                    Operator::Div => 1,
                                    Operator::Sum => 2,
                                    Operator::Sub => 2,
                                    Operator::Not => 3,
                                    Operator::Lt => 4,
                                    Operator::Gt => 4,
                                    Operator::Eq => 4,
                                    Operator::And => 5,
                                    Operator::Or => 6
                                }
                            }
                        }
                    }
                }
            }

        }
        // If token is an operand push it into the output
    }
}

fn push_operator_to_out(o: Operator, out: Vec<Exp>) -> Result<(), ()> {
    Result::Ok(())
}


