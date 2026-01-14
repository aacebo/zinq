mod binary_expr;
mod ref_expr;
mod unary_expr;

pub use binary_expr::*;
pub use ref_expr::*;
pub use unary_expr::*;

use crate::{Build, ExprId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SemaExpr {
    Binary(SemaBinaryExpr),
    Ref(SemaRefExpr),
    Unary(SemaUnaryExpr),
}

impl Build for zinq_syntax::expr::Expr {
    type Output = ExprId;

    fn build(&self, ctx: &mut crate::Context) -> zinq_error::Result<Self::Output> {
        match self {
            Self::Arithmetic(v) => v.build(ctx),
            _ => unimplemented!(),
        }
    }
}
