use crate::{Arena, expr::HirExpr, id::ExprId};

#[derive(Clone, PartialEq, Eq)]
pub struct Context {
    pub exprs: Arena<ExprId, HirExpr>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            exprs: Arena::new(),
        }
    }
}
