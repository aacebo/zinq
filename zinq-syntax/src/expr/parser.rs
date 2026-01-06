use zinq_error::Result;
use zinq_parse::Cursor;
use zinq_token::{And, AndAnd, Cmp, Comma, Dot, Eq, Ident, LParen, Not, OrOr, Punctuated, RParen};

use crate::expr::{
    AddrExpr, AssignExpr, CallExpr, CmpExpr, Expr, GroupExpr, IdentExpr, LiteralExpr, LogicalExpr,
    MemberExpr, NotExpr,
};

pub trait ExprParser {
    fn parse_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_assign_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_or_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_and_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_cmp_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_unary_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_postfix_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_primary_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
}

impl ExprParser for zinq_parse::ZinqParser {
    fn parse_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        self.parse_assign_expr(cursor)
    }

    fn parse_assign_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        let expr = self.parse_or_expr(cursor)?;

        if self.peek::<Eq>(cursor).unwrap_or(false) {
            let eq = self.parse::<Eq>(cursor)?;
            let right = self.parse_assign_expr(cursor)?;

            return Ok(AssignExpr::new(expr, eq, right).into());
        }

        Ok(expr)
    }

    fn parse_or_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        let mut expr = self.parse_and_expr(cursor)?;

        while self.peek::<OrOr>(cursor).unwrap_or(false) {
            let op = self.parse::<OrOr>(cursor)?;
            let right = self.parse_and_expr(cursor)?;

            expr = LogicalExpr::new(expr, op.into(), right).into();
        }

        Ok(expr)
    }

    fn parse_and_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        let mut expr = self.parse_cmp_expr(cursor)?;

        while self.peek::<AndAnd>(cursor).unwrap_or(false) {
            let op = self.parse::<AndAnd>(cursor)?;
            let right = self.parse_cmp_expr(cursor)?;

            expr = LogicalExpr::new(expr, op.into(), right).into();
        }

        Ok(expr)
    }

    fn parse_cmp_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        let mut expr = self.parse_unary_expr(cursor)?;

        while self.peek::<Cmp>(cursor).unwrap_or(false) {
            let op = self.parse::<Cmp>(cursor)?;
            let right = self.parse_unary_expr(cursor)?;

            expr = CmpExpr::new(expr, op, right).into();
        }

        Ok(expr)
    }

    fn parse_unary_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        if self.peek::<Not>(cursor).unwrap_or(false) {
            let not = self.parse::<Not>(cursor)?;
            let right = self.parse_unary_expr(cursor)?;

            return Ok(NotExpr::new(not, right).into());
        } else if self.peek::<And>(cursor).unwrap_or(false) {
            let and = self.parse::<And>(cursor)?;
            let right = self.parse_unary_expr(cursor)?;

            return Ok(AddrExpr::new(and, right).into());
        }

        self.parse_postfix_expr(cursor)
    }

    fn parse_postfix_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        let mut expr = self.parse_primary_expr(cursor)?.into();

        while !cursor.eof() {
            if self.peek::<LParen>(cursor).unwrap_or(false) {
                let left_paren = self.parse::<LParen>(cursor)?;
                let args = self.parse::<Punctuated<Expr, Comma>>(cursor)?;
                let right_paren = self.parse::<RParen>(cursor)?;

                expr = CallExpr::new(expr, left_paren, args, right_paren).into();
            } else if self.peek::<Dot>(cursor).unwrap_or(false) {
                let dot = self.parse::<Dot>(cursor)?;
                let name = self.parse::<Ident>(cursor)?;

                expr = MemberExpr::new(expr, dot, name).into();
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        if self.peek::<LiteralExpr>(cursor).unwrap_or(false) {
            return Ok(self.parse::<LiteralExpr>(cursor)?.into());
        }

        if self.peek::<IdentExpr>(cursor).unwrap_or(false) {
            return Ok(self.parse::<IdentExpr>(cursor)?.into());
        }

        if self.peek::<GroupExpr>(cursor).unwrap_or(false) {
            return Ok(self.parse::<GroupExpr>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }
}
