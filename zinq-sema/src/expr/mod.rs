mod binary_expr;
mod ref_expr;
mod unary_expr;

pub use binary_expr::*;
pub use ref_expr::*;
pub use unary_expr::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Binary(BinaryExpr),
    Ref(RefExpr),
    Unary(UnaryExpr),
}
