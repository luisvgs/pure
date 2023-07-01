#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Nil,
    Int(i32),
    Bool(bool),
    Symbol(String),
    Pair(Box<Expr>, Box<Expr>),
    Lambda(Vec<String>, Vec<Expr>),
    List(Vec<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Nil => write!(f, "nil"),
            Expr::Int(n) => write!(f, "{}", n),
            Expr::Bool(b) => match b {
                true => write!(f, "#t"),
                _ => write!(f, "#f"),
            },
            Expr::Pair(a, b) => write!(f, "({} . {})", a, b),
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::List(list) => {
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
