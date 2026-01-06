use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Span};
use zinq_token::{Dot, Ident};

use crate::expr::{Expr, MemberExpr, PostfixExpr, PrimaryExpr};

///
/// ```
/// Expr
///   -> AssignExpr
///      -> OrExpr
///         -> AndExpr
///            -> CmpExpr
///               -> FactorExpr
///                  -> MultExpr
///                     -> UnaryExpr
///                        -> PostfixExpr
///                           -> PrimaryExpr
/// ```
///
pub trait ExprParser {
    fn parse_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_assign_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_or_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_and_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_equal_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_cmp_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_unary_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_primary_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
}

impl ExprParser for zinq_parse::ZinqParser {
    fn parse_expr(&mut self, _: &mut Cursor) -> Result<Expr> {
        todo!()
    }

    fn parse_assign_expr(&mut self, _: &mut Cursor) -> Result<Expr> {
        todo!()
    }

    fn parse_or_expr(&mut self, _: &mut Cursor) -> Result<Expr> {
        todo!()
    }

    fn parse_and_expr(&mut self, _: &mut Cursor) -> Result<Expr> {
        todo!()
    }

    fn parse_equal_expr(&mut self, _: &mut Cursor) -> Result<Expr> {
        todo!()
    }

    fn parse_cmp_expr(&mut self, _: &mut Cursor) -> Result<Expr> {
        todo!()
    }

    fn parse_unary_expr(&mut self, _: &mut Cursor) -> Result<Expr> {
        todo!()
    }

    fn parse_primary_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        let mut expr = Expr::from(self.parse::<PrimaryExpr>(cursor)?);

        if let Expr::Primary(PrimaryExpr::Ident(_)) = &expr {
            while self.peek::<Dot>(cursor).unwrap_or(false) {
                let dot = self.parse::<Dot>(cursor)?;
                let name = self.parse::<Ident>(cursor)?;

                expr = PostfixExpr::from(MemberExpr {
                    span: Span::from_bounds(expr.span(), name.span()),
                    target: Box::new(expr),
                    dot,
                    name,
                })
                .into();
            }
        }

        Ok(expr.into())
    }
}
