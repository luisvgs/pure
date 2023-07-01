use linefeed::{Interface, ReadResult};
use std::cell::RefCell;
use std::rc::Rc;
mod ast;
use ast::Token;
mod env;
use env::Env;
mod expr;
use expr::*;

pub fn eval(program: &str, env: &mut Rc<RefCell<Env>>) -> Result<Expr, String> {
    let parsed_list = parse(program);
    if parsed_list.is_err() {
        return Err(format!("{}", parsed_list.err().unwrap()));
    }
    eval_obj(&parsed_list.unwrap(), env)
}

pub fn eval_symbol(s: String, env: &mut Rc<RefCell<Env>>) -> Result<Expr, String> {
    match env.borrow_mut().vars.get(&s) {
        Some(v) => Ok(v.clone()),
        None => Err("Unbound symbol".into()),
    }
}

pub fn eval_obj(obj: &Expr, env: &mut Rc<RefCell<Env>>) -> Result<Expr, String> {
    match obj {
        Expr::Nil => Ok(Expr::Nil),
        Expr::Int(n) => Ok(Expr::Int(*n)),
        Expr::Bool(b) => Ok(Expr::Bool(*b)),
        Expr::Lambda(_params, _body) => Ok(Expr::Nil),
        Expr::Symbol(s) => eval_symbol(s.to_string(), env),
        Expr::List(lst) => eval_list(lst, env),
        _ => unimplemented!(),
    }
}

pub fn eval_binary_op(list: &Vec<Expr>, env: &mut Rc<RefCell<Env>>) -> Result<Expr, String> {
    let op = list[0].clone();
    let lhs = eval_obj(&list[1].clone(), env)?;
    let rhs = eval_obj(&list[2].clone(), env)?;

    match op {
        Expr::Symbol(s) => match s.as_str() {
            "or" => {
                let (lval, rval) = match (lhs, rhs) {
                    (Expr::Bool(a), Expr::Bool(b)) => (a, b),
                    _ => unimplemented!(),
                };
                Ok(Expr::Bool(lval || rval))
            }
            "+" => {
                let (lval, rval) = match (lhs, rhs) {
                    (Expr::Int(a), Expr::Int(b)) => (a, b),
                    _ => unimplemented!(),
                };
                Ok(Expr::Int(lval + rval))
            }
            "-" => {
                let (lval, rval) = match (lhs, rhs) {
                    (Expr::Int(a), Expr::Int(b)) => (a, b),
                    _ => unimplemented!(),
                };
                Ok(Expr::Int(lval - rval))
            }
            "/" => {
                let (lval, rval) = match (lhs, rhs) {
                    (Expr::Int(a), Expr::Int(b)) => (a, b),
                    _ => unimplemented!(),
                };
                Ok(Expr::Int(lval / rval))
            }
            "<" => {
                let (lval, rval) = match (lhs, rhs) {
                    (Expr::Int(a), Expr::Int(b)) => (a, b),
                    _ => unimplemented!(),
                };
                Ok(Expr::Bool(lval < rval))
            }
            ">" => {
                let (lval, rval) = match (lhs, rhs) {
                    (Expr::Int(a), Expr::Int(b)) => (a, b),
                    _ => unimplemented!(),
                };
                Ok(Expr::Bool(lval > rval))
            }
            "=" => {
                let (lval, rval) = match (lhs, rhs) {
                    (Expr::Int(a), Expr::Int(b)) => (a, b),
                    _ => unimplemented!(),
                };
                Ok(Expr::Bool(lval == rval))
            }
            "!=" => {
                let (lval, rval) = match (lhs, rhs) {
                    (Expr::Int(a), Expr::Int(b)) => (a, b),
                    _ => unimplemented!(),
                };
                Ok(Expr::Bool(lval != rval))
            }
            _ => unreachable!(),
        },
        _ => unimplemented!(),
    }
}

pub fn eval_define(list: &Vec<Expr>, env: &mut Rc<RefCell<Env>>) -> Result<Expr, String> {
    let symbol = match &list[1] {
        Expr::Symbol(name) => name,
        _ => unreachable!(),
    };

    let value = eval_obj(&list[2], env)?;

    env.borrow_mut().vars.insert(symbol.into(), value);

    Ok(Expr::Nil)
}
pub fn eval_function_call(
    s: String,
    list: &Vec<Expr>,
    env: &mut Rc<RefCell<Env>>,
) -> Result<Expr, String> {
    let binding = env.borrow_mut();

    let lamdba = binding.vars.get(&s);
    if lamdba.is_none() {
        return Err(format!("Unbound symbol: {}", s));
    }

    let func = lamdba.unwrap();

    match func {
        Expr::Lambda(params, body) => {
            let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
            for (i, param) in params.iter().enumerate() {
                let val = eval_obj(&list[i + 1], &mut env.clone())?;
                new_env.borrow_mut().vars.insert(param.clone(), val);
            }
            return eval_obj(&Expr::List(body.to_vec()), &mut new_env);
        }
        _ => return Err(format!("Not a lambda: {}", s)),
    }
}
pub fn eval_function_definition(
    list: &Vec<Expr>,
    _env: &mut Rc<RefCell<Env>>,
) -> Result<Expr, String> {
    let args = match &list[1] {
        Expr::List(el) => {
            let mut args = Vec::new();
            for arg in el {
                match arg {
                    Expr::Symbol(name) => args.push(name.clone()),
                    _ => unreachable!(),
                }
            }

            args
        }
        _ => return Err("Expected (lambda) definition".into()),
    };

    let body = match &list[2] {
        Expr::List(list) => list.clone(),
        _ => unreachable!(),
    };

    Ok(Expr::Lambda(args, body))
}

pub fn eval_if(list: &Vec<Expr>, env: &mut Rc<RefCell<Env>>) -> Result<Expr, String> {
    let cond_obj = eval_obj(&list[1], env)?;
    let cond = match cond_obj {
        Expr::Bool(b) => b,
        _ => return Err("Must be boolean".into()),
    };
    // if cond t f
    match cond {
        true => eval_obj(&list[2], env),
        _ => eval_obj(&list[3], env),
    }
}
pub fn eval_list(list: &Vec<Expr>, env: &mut Rc<RefCell<Env>>) -> Result<Expr, String> {
    let head = &list[0];
    match head {
        Expr::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" | "or" => {
                return eval_binary_op(&list, env);
            }
            "val" => eval_define(&list, env),
            "if" => eval_if(&list, env),
            "pair" => eval_pair(&list, env),
            "lambda" => eval_function_definition(&list, env),
            _ => eval_function_call(s.to_string(), &list, env),
        },
        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let result = eval_obj(obj, env)?;
                match result {
                    Expr::Nil => {}
                    _ => new_list.push(result),
                }
            }
            Ok(Expr::List(new_list))
        }
    }
}
pub fn eval_pair(list: &Vec<Expr>, env: &mut Rc<RefCell<Env>>) -> Result<Expr, String> {
    let lhs = eval_obj(&list[1].clone(), env)?;
    let rhs = eval_obj(&list[2].clone(), env)?;

    Ok(Expr::Pair(Box::new(lhs), Box::new(rhs)))
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
            "nil" => tokens.push(Token::Nil),
            x if word.parse::<i32>().is_ok() => tokens.push(Token::Int(x.parse::<i32>().unwrap())),
            _ => tokens.push(Token::Symbol(word.to_string())),
        }
    }

    Ok(tokens)
}

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = Interface::new("~> ").unwrap();
    let mut env = Env::new();

    reader.set_prompt(format!("{}", "~> ").as_ref()).unwrap();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq(":q") {
            break;
        }
        let val = eval(input.as_ref(), &mut env)?;
        match val {
            Expr::Nil => {}
            Expr::Int(n) => println!("{}", n),
            Expr::Bool(b) => println!("{}", b),
            Expr::Symbol(s) => println!("{}", s),
            Expr::Lambda(params, body) => {
                println!("Lambda(");
                for param in params {
                    println!("{} ", param);
                }
                println!(")");
                for expr in body {
                    println!(" {}", expr);
                }
            }
            _ => println!("{}", val),
        }
    }

    println!("Good bye");
    Ok(())
}

#[test]
fn should_parse_expressions() {
    let cases = [
        ("(1)", Expr::List(vec![Expr::Int(1)])),
        ("(2)", Expr::List(vec![Expr::Int(2)])),
        ("(nil)", Expr::List(vec![Expr::Nil])),
        ("(#t)", Expr::List(vec![Expr::Bool(true)])),
        ("(#f)", Expr::List(vec![Expr::Bool(false)])),
        ("(1 2)", Expr::List(vec![Expr::Int(1), Expr::Int(2)])),
    ];
    for (input, expected) in cases.into_iter() {
        let parsed: Expr = parse(input).unwrap();
        assert_eq!(parsed, expected);
    }
}
#[test]
fn should_eval_expressions() {
    let mut env = Env::new();
    let cases = [
        ("(+ 2 1)", Expr::Int(3)),
        ("(- 9 3)", Expr::Int(6)),
        ("(/ 10 2)", Expr::Int(5)),
        ("(= 3 3)", Expr::Bool(true)),
        ("(= 3 18)", Expr::Bool(false)),
        ("(< 30 70)", Expr::Bool(true)),
        ("(< 30 10)", Expr::Bool(false)),
        ("(> 67 40)", Expr::Bool(true)),
        ("(> 58 70)", Expr::Bool(false)),
        ("(or #t #f)", Expr::Bool(true)),
        ("(or #t #t)", Expr::Bool(true)),
        (
            "(pair 3 2)",
            Expr::Pair(Box::new(Expr::Int(3)), Box::new(Expr::Int(2))),
        ),
        ("((val x 10) (+ x x) )", Expr::List(vec![Expr::Int(20)])),
    ];
    for (input, expected) in cases.into_iter() {
        let parsed: Expr = eval_obj(&parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(parsed, expected);
    }
}
