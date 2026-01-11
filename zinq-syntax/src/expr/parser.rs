use zinq_error::Result;
use zinq_parse::Cursor;
use zinq_token::{
    And, AndAnd, Arithmetic, Cmp, Colon, Comma, Dot, Eq, Ident, Is, LBrace, LParen, Match, Minus,
    Mut, Not, OrOr, Plus, Punctuated, Question, RBrace, RParen, Slash, Star,
};

use crate::{
    expr::{
        ArithmeticExpr, Arm, ArrayExpr, AssignExpr, CallExpr, CmpExpr, Expr, GroupExpr, IfExpr,
        IsExpr, LiteralExpr, LogicalExpr, MatchExpr, MemberExpr, NegExpr, NotExpr, PathExpr,
        RefExpr, StructExpr, TupleExpr,
    },
    ty::Type,
};

pub trait ExprParser {
    fn parse_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_assign_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_match_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_if_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_or_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_and_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_is_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_cmp_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_term_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_factor_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_unary_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_prefix_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_postfix_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
    fn parse_primary_expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;
}

impl ExprParser for zinq_parse::ZinqParser {
    fn parse_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        self.parse_assign_expr(cursor)
    }

    fn parse_assign_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        let expr = self.parse_match_expr(cursor)?;

        if self.peek::<Eq>(cursor).unwrap_or(false) {
            let eq = self.parse::<Eq>(cursor)?;
            let right = self.parse_assign_expr(cursor)?;

            return Ok(AssignExpr::new(expr, eq, right).into());
        }

        Ok(expr)
    }

    fn parse_match_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        if self.peek::<Match>(cursor).unwrap_or(false) {
            let keyword = self.parse::<Match>(cursor)?;
            let expr = self.parse_assign_expr(cursor)?;
            let left_brace = self.parse::<LBrace>(cursor)?;
            let arms = self.parse::<Punctuated<Arm, Comma>>(cursor)?;
            let right_brace = self.parse::<RBrace>(cursor)?;

            return Ok(MatchExpr {
                keyword,
                expr: Box::new(expr),
                left_brace,
                arms,
                right_brace,
            }
            .into());
        }

        Ok(self.parse_if_expr(cursor)?)
    }

    fn parse_if_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        let expr = self.parse_or_expr(cursor)?;

        if self.peek::<Question>(cursor).unwrap_or(false) {
            let question = self.parse::<Question>(cursor)?;
            let then_expr = self.parse_assign_expr(cursor)?;
            let colon = self.parse::<Colon>(cursor)?;
            let else_expr = self.parse_assign_expr(cursor)?;

            return Ok(IfExpr {
                cond: Box::new(expr),
                question,
                then_expr: Box::new(then_expr),
                colon,
                else_expr: Box::new(else_expr),
            }
            .into());
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
        let mut expr = self.parse_is_expr(cursor)?;

        while self.peek::<AndAnd>(cursor).unwrap_or(false) {
            let op = self.parse::<AndAnd>(cursor)?;
            let right = self.parse_is_expr(cursor)?;

            expr = LogicalExpr::new(expr, op.into(), right).into();
        }

        Ok(expr)
    }

    fn parse_is_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        let mut expr = self.parse_cmp_expr(cursor)?;

        while self.peek::<Is>(cursor).unwrap_or(false) {
            let keyword = self.parse::<Is>(cursor)?;
            let ty = self.parse::<Type>(cursor)?;

            expr = IsExpr::new(expr, keyword, ty).into();
        }

        Ok(expr)
    }

    fn parse_cmp_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        let mut expr = self.parse_term_expr(cursor)?;

        while self.peek::<Cmp>(cursor).unwrap_or(false) {
            let op = self.parse::<Cmp>(cursor)?;
            let right = self.parse_term_expr(cursor)?;

            expr = CmpExpr::new(expr, op, right).into();
        }

        Ok(expr)
    }

    fn parse_term_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        let mut expr = self.parse_factor_expr(cursor)?;

        while self.peek::<Plus>(cursor).unwrap_or(false)
            || self.peek::<Minus>(cursor).unwrap_or(false)
        {
            let op = self.parse::<Arithmetic>(cursor)?;
            let right = self.parse_factor_expr(cursor)?;

            expr = ArithmeticExpr::new(expr, op, right).into();
        }

        Ok(expr)
    }

    fn parse_factor_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        let mut expr = self.parse_unary_expr(cursor)?;

        while self.peek::<Star>(cursor).unwrap_or(false)
            || self.peek::<Slash>(cursor).unwrap_or(false)
        {
            let op = self.parse::<Arithmetic>(cursor)?;
            let right = self.parse_unary_expr(cursor)?;

            expr = ArithmeticExpr::new(expr, op, right).into();
        }

        Ok(expr)
    }

    fn parse_unary_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        if self.peek::<Not>(cursor).unwrap_or(false) {
            let not = self.parse::<Not>(cursor)?;
            let right = self.parse_unary_expr(cursor)?;

            return Ok(NotExpr::new(not, right).into());
        } else if self.peek::<Minus>(cursor).unwrap_or(false) {
            let minus = self.parse::<Minus>(cursor)?;
            let right = self.parse_unary_expr(cursor)?;

            return Ok(NegExpr::new(minus, right).into());
        }

        self.parse_prefix_expr(cursor)
    }

    fn parse_prefix_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        if self.peek::<And>(cursor).unwrap_or(false) {
            let and = self.parse::<And>(cursor)?;
            let mut mutable = None;

            if self.peek::<Mut>(cursor).unwrap_or(false) {
                mutable = Some(self.parse::<Mut>(cursor)?);
            }

            let right = self.parse_unary_expr(cursor)?;
            return Ok(RefExpr::new(and, mutable, right).into());
        }

        self.parse_postfix_expr(cursor)
    }

    fn parse_postfix_expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        let mut expr = self.parse_primary_expr(cursor)?;

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

        if self.peek::<ArrayExpr>(cursor).unwrap_or(false) {
            return Ok(self.parse::<ArrayExpr>(cursor)?.into());
        }

        if self.peek::<TupleExpr>(cursor).unwrap_or(false) {
            return Ok(self.parse::<TupleExpr>(cursor)?.into());
        }

        if self.peek::<StructExpr>(cursor).unwrap_or(false) {
            return Ok(self.parse::<StructExpr>(cursor)?.into());
        }

        if self.peek::<PathExpr>(cursor).unwrap_or(false) {
            return Ok(self.parse::<PathExpr>(cursor)?.into());
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
