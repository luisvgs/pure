use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
pub enum Token {
    Int(i32),
    Symbol(String),
    RParen,
    LParen,
    True,
    False,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Void,
    Int(i32),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

#[derive(Debug, Default)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    vars: std::collections::HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Env::default()))
    }
}

pub fn eval_symbol(s: String, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    match env.borrow_mut().vars.get(&s) {
        Some(v) => Ok(v.clone()),
        None => Err("Unbound symbol".into()),
    }
}

pub fn eval_obj(obj: &Object, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    match obj {
        Object::Void => Ok(Object::Void),
        Object::Int(n) => Ok(Object::Int(*n)),
        Object::Bool(b) => Ok(Object::Bool(*b)),
        Object::Lambda(_params, _body) => Ok(Object::Void),
        Object::Symbol(s) => eval_symbol(s.to_string(), env),
        Object::List(lst) => eval_list(lst, env),
    }
}

pub fn eval_binary_op(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let op = list[0].clone();
    let lhs = eval_obj(&list[1].clone(), env)?;
    let rhs = eval_obj(&list[2].clone(), env)?;

    let (lval, rval) = match (lhs, rhs) {
        (Object::Int(a), Object::Int(b)) => (a, b),
        _ => unimplemented!(),
    };
    match op {
        Object::Symbol(s) => match s.as_str() {
            "+" => Ok(Object::Int(rval + lval)),
            "-" => Ok(Object::Int(lval - rval)),
            "/" => Ok(Object::Int(lval / rval)),
            "<" => Ok(Object::Bool(lval < rval)),
            ">" => Ok(Object::Bool(lval > rval)),
            "=" => Ok(Object::Bool(lval == rval)),
            "!=" => Ok(Object::Bool(lval != rval)),
            _ => unreachable!(),
        },
        _ => unimplemented!(),
    }
}
pub fn eval_define(obj: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    todo!()
}
pub fn eval_function_call(
    s: String,
    obj: &Vec<Object>,
    env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
    todo!()
}
pub fn eval_function_definition(
    obj: &Vec<Object>,
    env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
    todo!()
}

pub fn eval_list(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let head = &list[0];
    match head {
        Object::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => {
                return eval_binary_op(&list, env);
            }
            "define" => eval_define(&list, env),
            // "if" => eval_if(&list, env),
            "lambda" => eval_function_definition(&list, env),
            _ => eval_function_call(s.to_string(), &list, env),
        },
        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let result = eval_obj(obj, env)?;
                match result {
                    Object::Void => {}
                    _ => new_list.push(result),
                }
            }
            Ok(Object::List(new_list))
        }
    }
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
            s @ ("+" | "-" | "<" | ">" | "/" | "=" | "!=") => {
                tokens.push(Token::Symbol(s.to_string()))
            }
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
#[test]
fn should_eval_expressions() {
    let mut env = Env::new();
    let cases = [
        ("(+ 2 1)", Object::Int(3)),
        ("(- 9 3)", Object::Int(6)),
        ("(/ 10 2)", Object::Int(5)),
        ("(= 3 3)", Object::Bool(true)),
        ("(= 3 18)", Object::Bool(false)),
        ("(< 30 70)", Object::Bool(true)),
        ("(< 30 10)", Object::Bool(false)),
        ("(> 67 40)", Object::Bool(true)),
        ("(> 58 70)", Object::Bool(false)),
    ];
    for (input, expected) in cases.into_iter() {
        let parsed: Object = eval_obj(&parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(parsed, expected);
    }
}
