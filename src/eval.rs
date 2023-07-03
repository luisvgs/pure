use crate::env::*;
use crate::expr::*;
use crate::parser::*;
use std::cell::RefCell;
use std::rc::Rc;

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

pub fn eval_empty(list: &Vec<Expr>) -> Result<Expr, String> {
    let mut dummy = false;
    if let Expr::List(list_val) = &list[1] {
        if list_val.len() == 0 {
            dummy = true
        }
    }
    Ok(Expr::Bool(dummy))
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
            "empty?" => eval_empty(&list),
            "pair" => eval_pair(&list, env),
            "fn" => eval_function_definition(&list, env),
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
