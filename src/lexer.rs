use crate::token::Token;
use crate::token::Operand;
use crate::token::Operator;

pub fn make_token(word: &String) -> Result<Token, ()> {
    let token = match word.as_str() {
        "true" => Token::Operand(Operand::Bool(true)),
        "false" => Token::Operand(Operand::Bool(false)),
        "null" => Token::Operand(Operand::Null),
        ";" => Token::Operator(Operator::Seq),
        "=" => Token::Operator(Operator::Assign),
        "&&" => Token::Operator(Operator::And),
        "||" => Token::Operator(Operator::Or),
        "!" => Token::Operator(Operator::Not),
        "==" => Token::Operator(Operator::Eq),
        "!=" => Token::Operator(Operator::Neq),
        "<" => Token::Operator(Operator::Lt),
        ">" => Token::Operator(Operator::Gt),
        "+" => Token::Operator(Operator::Sum),
        "-" => Token::Operator(Operator::Sub),
        "*" => Token::Operator(Operator::Mul),
        "/" => Token::Operator(Operator::Div),
        "let" => Token::Let,
        "fn" => Token::Fn,
        "if" => Token::If,
        "else" => Token:: Else,
        "(" => Token::RoundBracketOpen,
        ")" => Token::RoundBracketClosed,
        "{" => Token::CurlyBracketOpen,
        "}" => Token::CurlyBracketClosed,
        "," => Token::Comma,
        s => match s.parse::<i32>() {
            Result::Ok(i) => Token::Operand(Operand::Int(i)),
            Result::Err(_) => Token::Operand(Operand::Var(String::from(s)))
        }
    };
    Result::Ok(token)
}



pub fn tokenize(text: String) -> Result<Vec<Token>, ()> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut token_buffer = String::from("");

    for c in text.chars() {
        match c {
            ' ' | '\n' | '\t' => {
                flush_token_buffer(&mut token_buffer, &mut tokens)
            }
            ';' => {
                flush_token_buffer(&mut token_buffer, &mut tokens);
                tokens.push(Token::Operator(Operator::Seq));
            }
            '+' => {
                flush_token_buffer(&mut token_buffer, &mut tokens);
                tokens.push(Token::Operator(Operator::Sum));
            }
            '-' => {
                flush_token_buffer(&mut token_buffer, &mut tokens);
                tokens.push(Token::Operator(Operator::Sub));
            }
            '*' => {
                flush_token_buffer(&mut token_buffer, &mut tokens);
                tokens.push(Token::Operator(Operator::Mul));
            }
            '\\' => {
                flush_token_buffer(&mut token_buffer, &mut tokens);
                tokens.push(Token::Operator(Operator::Div));
            }
            '(' => {
                flush_token_buffer(&mut token_buffer, &mut tokens);
                tokens.push(Token::RoundBracketOpen);
            }
            ')' => {
                flush_token_buffer(&mut token_buffer, &mut tokens);
                tokens.push(Token::RoundBracketClosed);
            }
            '{' => {
                flush_token_buffer(&mut token_buffer, &mut tokens);
                tokens.push(Token::CurlyBracketOpen);
            }
            '}' => {
                flush_token_buffer(&mut token_buffer, &mut tokens);
                tokens.push(Token::CurlyBracketClosed);
            }
            c => token_buffer.push(c)
        }
    }
    
    flush_token_buffer(&mut token_buffer, &mut tokens);
    tokens.reverse();
    Result::Ok(tokens)
}


fn flush_token_buffer(buffer: &mut String, tokens: &mut Vec<Token>) {
    if buffer.len() > 0 {
        tokens.push(make_token(buffer).unwrap());
        buffer.clear();
    }
}
