use crate::{expr::SemaExpr, id::ExprId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SemaLiteralExpr {
    pub inner: ExprId,
}

impl From<SemaLiteralExpr> for SemaExpr {
    fn from(value: SemaLiteralExpr) -> Self {
        Self::Literal(value)
    }
}

// impl Build for zinq_syntax::expr::LiteralExpr {
//     type Output = ExprId;

//     fn build(&self, ctx: &mut crate::Context) -> zinq_error::Result<Self::Output> {
//         let id = ExprId::new(self.name())
//             .attr(self.value.to_string().into())
//             .build()
//             .into();

//         ctx.exprs
//             .add(id, SemaLiteralExpr {  }.into(), self.span());

//         Ok(id)
//     }
// }
