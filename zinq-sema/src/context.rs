use crate::{Arena, ExprId, expr::SemaExpr};

#[derive(Clone, PartialEq, Eq)]
pub struct Context {
    pub exprs: Arena<ExprId, SemaExpr>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            exprs: Arena::new(),
        }
    }
}
