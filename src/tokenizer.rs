use crate::ast::*;

pub fn tokenize(program: &str) -> Result<Vec<Token>, String> {
    let foo = program.replace("(", " ( ").replace(")", " ) ");
    let words = foo.split_whitespace();

    let mut tokens: Vec<Token> = Vec::new();

    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            "'" => tokens.push(Token::Quote),
            "#t" => tokens.push(Token::True),
            "#f" => tokens.push(Token::False),
            "nil" => tokens.push(Token::Nil),
            x if word.parse::<i32>().is_ok() => tokens.push(Token::Int(x.parse::<i32>().unwrap())),
            _ => tokens.push(Token::Symbol(word.to_string())),
        }
    }

    Ok(tokens)
}
