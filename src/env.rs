use crate::Expr;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Env {
    pub parent: Option<Rc<RefCell<Env>>>,
    pub vars: std::collections::HashMap<String, Expr>,
}

impl Env {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Env::default()))
    }

    pub fn extend(env: Rc<RefCell<Self>>) -> Self {
        Self {
            parent: Some(env),
            vars: std::collections::HashMap::new(),
        }
    }
}
