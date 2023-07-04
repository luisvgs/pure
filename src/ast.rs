#[derive(Debug, PartialEq)]
pub enum Token {
    Int(i32),
    Symbol(String),
    RParen,
    LParen,
    True,
    False,
    Nil,
    Quote,
}
