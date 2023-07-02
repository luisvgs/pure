use linefeed::{Interface, ReadResult};

mod ast;
mod env;
use env::Env;
mod expr;
use expr::*;
mod eval;
use eval::*;
mod parser;
mod tokenizer;

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
    Ok(())
}

#[test]
fn should_parse_expressions() {
    use crate::parser::parse;
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
    use crate::parser::parse;
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
#[test]
fn should_eval_functions() {
    use crate::parser::parse;
    let mut env = Env::new();
    let cases = [(
        "(
        (val foo
            (fn (x y)
            (+ x y))
        )
        (foo 10 20)
        )",
        Expr::List(vec![Expr::Int(30)]),
    )];
    for (input, expected) in cases.into_iter() {
        let parsed: Expr = eval_obj(&parse(input).unwrap(), &mut env).unwrap();
        assert_eq!(parsed, expected);
    }
}
