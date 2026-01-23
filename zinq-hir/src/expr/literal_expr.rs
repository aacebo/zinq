use crate::expr::HirExpr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirLiteralExpr {}

impl From<HirLiteralExpr> for HirExpr {
    fn from(value: HirLiteralExpr) -> Self {
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
//             .add(id, HirLiteralExpr {  }.into(), self.span());

//         Ok(id)
//     }
// }
