#[derive(PartialEq)]
pub enum Token {
    Int(i32),
    Symbol(String),
    RParen,
    LParen,
    True,
    False,
}

#[derive(Debug, PartialEq)]
pub enum Object {
    Void,
    Int(i32),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Void => write!(f, "void"),
            Object::Int(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Symbol(s) => write!(f, "{}", s),
            Object::List(list) => {
                write!(f, "(")?;
                for (i, obj) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", obj)?;
                }
                write!(f, ")")
            }
            _ => todo!(),
        }
    }
}

pub fn tokenize(program: &str) -> Result<Vec<Token>, String> {
    let foo = program.replace("(", " ( ").replace(")", " ) ");
    let words = foo.split_whitespace();

    let mut tokens: Vec<Token> = Vec::new();

    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            "#t" => tokens.push(Token::True),
            "#f" => tokens.push(Token::False),
            x if word.parse::<i32>().is_ok() => tokens.push(Token::Int(x.parse::<i32>().unwrap())),
            _ => todo!(),
        }
    }

    Ok(tokens)
}

pub fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, String> {
    let token = tokens.pop();
    if token != Some(Token::LParen) {
        return Err("WHoppsies".into());
    }

    let mut list: Vec<Object> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop();
        if token == None {
            return Err("WHoppsies".into());
        }
        let t = token.unwrap();
        match t {
            Token::Int(n) => list.push(Object::Int(n)),
            Token::Symbol(s) => list.push(Object::Symbol(s)),
            Token::False => list.push(Object::Bool(false)),
            Token::True => list.push(Object::Bool(true)),
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list(tokens)?;
                list.push(sub_list);
            }
            Token::RParen => {
                return Ok(Object::List(list));
            }
        }
    }

    Ok(Object::List(list))
}

pub fn parse(tokens: &str) -> Result<Object, String> {
    let token_result = tokenize(tokens);

    if token_result.is_err() {
        return Err("Whoops".into());
    }
    let mut tokens_ = token_result.unwrap().into_iter().rev().collect::<Vec<_>>();
    let parsed_list = parse_list(&mut tokens_)?;

    Ok(parsed_list)
}

fn main() {}

#[test]
fn should_parse_expressions() {
    let cases = [
        ("(1)", Object::List(vec![Object::Int(1)])),
        ("(2)", Object::List(vec![Object::Int(2)])),
        ("(#t)", Object::List(vec![Object::Bool(true)])),
        ("(#f)", Object::List(vec![Object::Bool(false)])),
        ("(1 2)", Object::List(vec![Object::Int(1), Object::Int(2)])),
    ];
    for (input, expected) in cases.into_iter() {
        let parsed: Object = parse(input).unwrap();
        assert_eq!(parsed, expected);
    }
}
