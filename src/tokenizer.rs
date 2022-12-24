use crate::token::Token;
use crate::token::Operand;
use crate::token::Operator;

pub fn tokenize(text: String) -> Result<Vec<Token>, ()> {
    let mut tokens: Vec<Token> = Vec::new();
    let words = text.split_whitespace();
    for word in words {
        let token: Token = match word {
            ";" => Token::Operator(Operator::Seq),
            "=" => Token::Operator(Operator::Assign),
            "&&" => Token::Operator(Operator::And),
            "||" => Token::Operator(Operator::Or),
            "!" => Token::Operator(Operator::Not),
            "==" => Token::Operator(Operator::Eq),
            "<" => Token::Operator(Operator::Lt),
            ">" => Token::Operator(Operator::Gt),
            "+" => Token::Operator(Operator::Sum),
            "-" => Token::Operator(Operator::Sub),
            s => {
                match s.parse::<i32>() {
                    Result::Ok(i) => Token::Operand(Operand::Int(i)),
                    Result::Err(_) => Token::Operand(Operand::Var(String::from(s)))
                }
            }
        };
        tokens.push(token);
    }
    tokens.reverse();
    Result::Ok(tokens)
}
