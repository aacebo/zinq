use zinq_parse::Spanned;
use zinq_syntax::Syntax;
use zinq_token::{Arithmetic, Cmp, Logical};

use crate::{Build, expr::HirExpr, id::ExprId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirBinaryExpr {
    pub left: ExprId,
    pub op: BinaryOp,
    pub right: ExprId,
}

impl From<HirBinaryExpr> for HirExpr {
    fn from(value: HirBinaryExpr) -> Self {
        Self::Binary(value)
    }
}

impl Build for zinq_syntax::expr::ArithmeticExpr {
    type Output = ExprId;

    fn build(&self, ctx: &mut crate::Context) -> zinq_error::Result<Self::Output> {
        let left = self.left.build(ctx)?;
        let right = self.right.build(ctx)?;
        let op = BinaryOp::from(self.op.clone());
        let id = zinq_hash::v1()
            .push_str(self.name())
            .push_field("left", &left.to_string())
            .push_field("op", op as u8)
            .push_field("right", &right.to_string())
            .build()
            .into();

        ctx.exprs
            .add(id, HirBinaryExpr { left, op, right }.into(), self.span());

        Ok(id)
    }
}

#[repr(u8)]
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

// #[cfg(test)]
// mod tests {
//     use zinq_error::Result;
//     use zinq_parse::Span;
//     use zinq_syntax::expr::ExprParser;

//     use crate::{Build, Context};

//     #[test]
//     fn should_build_context() -> Result<()> {
//         let mut parser = zinq_parse::ZinqParser;
//         let mut cursor = Span::from_bytes(b"a + 2").cursor();
//         let expr = parser.parse_expr(&mut cursor)?;

//         debug_assert_eq!(expr.to_string(), "a + 2");
//         debug_assert!(expr.is_arithmetic());

//         let mut ctx = Context::new();
//         let id = expr.build(&mut ctx)?;

//         debug_assert_eq!(id.to_string(), "V1::Expr::Binary::Arithmetic");
//         Ok(())
//     }
// }
