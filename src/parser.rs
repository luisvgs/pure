use crate::{ast::*, expr::*, tokenizer::*};

pub fn parse_list(tokens: &mut Vec<Token>) -> Result<Expr, String> {
    let token = tokens.pop();
    if token != Some(Token::LParen) {
        return Err("Expected (.".into());
    }

    let mut list: Vec<Expr> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop();
        if token == None {
            return Err("Tokenization error".into());
        }
        let t = token.unwrap();
        match t {
            Token::Int(n) => list.push(Expr::Int(n)),
            Token::Symbol(s) => list.push(Expr::Symbol(s)),
            Token::False => list.push(Expr::Bool(false)),
            Token::True => list.push(Expr::Bool(true)),
            Token::Nil => list.push(Expr::Nil),
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list(tokens)?;
                list.push(sub_list);
            }
            Token::RParen => {
                return Ok(Expr::List(list));
            }
            Token::Quote => continue,
        }
    }

    Ok(Expr::List(list))
}

pub fn parse(tokens: &str) -> Result<Expr, String> {
    let token_result = tokenize(tokens);

    if token_result.is_err() {
        return Err("Token error".into());
    }
    let mut tokens_ = token_result.unwrap().into_iter().rev().collect::<Vec<_>>();
    let parsed_list = parse_list(&mut tokens_)?;

    Ok(parsed_list)
}
