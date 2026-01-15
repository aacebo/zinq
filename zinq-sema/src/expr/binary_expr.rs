use zinq_parse::Spanned;
use zinq_token::{Arithmetic, Cmp, Logical};

use crate::{Build, ExprId, expr::SemaExpr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SemaBinaryExpr {
    pub left: ExprId,
    pub op: BinaryOp,
    pub right: ExprId,
}

impl From<SemaBinaryExpr> for SemaExpr {
    fn from(value: SemaBinaryExpr) -> Self {
        Self::Binary(value)
    }
}

impl Build for zinq_syntax::expr::ArithmeticExpr {
    type Output = ExprId;

    fn build(&self, ctx: &mut crate::Context) -> zinq_error::Result<Self::Output> {
        let left = self.left.build(ctx)?;
        let right = self.right.build(ctx)?;
        let id = zinq_syntax::expr::Expr::from(self.clone()).into();

        ctx.exprs.add(
            id,
            SemaBinaryExpr {
                left,
                op: self.op.clone().into(),
                right,
            }
            .into(),
            self.span(),
        );

        Ok(id)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Div,
    Mult,

    // Compare
    Eq,
    NotEq,
    Gt,
    GtEq,
    Lt,
    LtEq,

    // Logical
    And,
    Or,
}

impl From<Arithmetic> for BinaryOp {
    fn from(value: Arithmetic) -> Self {
        match value {
            Arithmetic::Add(_) => Self::Add,
            Arithmetic::Sub(_) => Self::Sub,
            Arithmetic::Div(_) => Self::Div,
            Arithmetic::Mult(_) => Self::Mult,
        }
    }
}

impl From<Cmp> for BinaryOp {
    fn from(value: Cmp) -> Self {
        match value {
            Cmp::Eq(_) => Self::Eq,
            Cmp::NotEq(_) => Self::NotEq,
            Cmp::Gt(_) => Self::Gt,
            Cmp::GtEq(_) => Self::GtEq,
            Cmp::Lt(_) => Self::Lt,
            Cmp::LtEq(_) => Self::LtEq,
        }
    }
}

impl From<Logical> for BinaryOp {
    fn from(value: Logical) -> Self {
        match value {
            Logical::And(_) => Self::And,
            Logical::Or(_) => Self::Or,
        }
    }
}
