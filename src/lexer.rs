use crate::token::Token;
use crate::token::Operand;
use crate::token::Operator;

pub fn tokenize(text: String) -> Result<Vec<Token>, ()> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer = String::from("");

    let mut callable = false;

    let mut chars = text.chars().peekable();
    loop {
        match chars.next() {
            Option::None => break,

            Option::Some(' ' | '\n' | '\t') => {
                flush_buffer(&mut buffer, &mut tokens, &mut callable)?;
            },

            Option::Some('(') => {
                flush_buffer(&mut buffer, &mut tokens, &mut callable)?;
                let token: Token = if callable {Token::FunctionCallOpen} else {Token::RoundBracketOpen};
                callable = token.is_callable();
                tokens.push(token)
            }

            Option::Some(c) if [
                ';', ',', '+', '-', '*', '/', '[', ']', '{', '}', ')', '<', '>'
            ].contains(&c) => {
                flush_buffer(&mut buffer, &mut tokens, &mut callable)?;
                let token = make_token(&c.to_string())?;
                callable = token.is_callable();
                tokens.push(token)
            },

            Option::Some('=') => {
                flush_buffer(&mut buffer, &mut tokens, &mut callable)?;
                let token = match chars.peek() {
                    Option::Some('=') => {
                        chars.next();
                        Token::Operator(Operator::Eq)
                    },
                    _ => Token::Operator(Operator::Assign)
                };
                callable = token.is_callable();
                tokens.push(token)
            },

            Option::Some('!') => {
                flush_buffer(&mut buffer, &mut tokens, &mut callable)?;
                let token = match chars.peek() {
                    Option::Some('=') => {
                        chars.next();
                        Token::Operator(Operator::Neq)
                    },
                    _ =>Token::Operator(Operator::Not)
                };
                callable = token.is_callable();
                tokens.push(token)
            },

            Option::Some('&') => {
                flush_buffer(&mut buffer, &mut tokens, &mut callable)?;
                let token = match chars.next() {
                    Option::Some('&') => {
                        Token::Operator(Operator::And)
                    },
                    _ => return Result::Err(())
                };
                callable = token.is_callable();
                tokens.push(token)
            },

            Option::Some('|') => {
                flush_buffer(&mut buffer, &mut tokens, &mut callable)?;
                let token = match chars.next() {
                    Option::Some('|') => {
                        Token::Operator(Operator::Or)
                    },
                    _ => return Result::Err(())
                };
                callable = token.is_callable();
                tokens.push(token)
            },

            Option::Some(c) => buffer.push(c)
        }
    };
    flush_buffer(&mut buffer, &mut tokens, &mut callable)?;
    tokens.reverse();
    Result::Ok(tokens)
}

fn flush_buffer(buffer: &mut String, tokens: &mut Vec<Token>, callable: &mut bool) -> Result<(), ()> {
    if !buffer.is_empty() {
        let token: Token = make_token(&buffer)?;
        buffer.clear();
        *callable = token.is_callable();
        tokens.push(token);
    };
    Result::Ok(())
}

fn make_token(word: &String) -> Result<Token, ()> {
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
        "[" => Token::SquareBracketOpen,
        "]" => Token::SquareBracketClosed,
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
