use zinq_error::Result;
use zinq_parse::{Cursor, Parser};
use zinq_token::{Token, TokenParser};

use crate::{Visibility, expr::Expr, stmt::Stmt, ty::Type};

pub trait SyntaxParser: Parser<Item = Token> {
    fn is_expr(&self, cursor: &Cursor) -> bool;
    fn expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;

    fn is_stmt(&self, cursor: &Cursor) -> bool;
    fn stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;

    fn is_type(&self, cursor: &Cursor) -> bool;
    fn ty(&mut self, cursor: &mut Cursor) -> Result<Type>;

    fn is_vis(&self, cursor: &Cursor) -> bool;
    fn vis(&mut self, cursor: &mut Cursor) -> Result<Visibility>;
}

impl SyntaxParser for TokenParser {
    fn is_expr(&self, cursor: &Cursor) -> bool {
        self.peek_as::<Expr>(cursor).unwrap_or(false)
    }

    fn expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        self.parse_as::<Expr>(cursor)
    }

    fn is_stmt(&self, cursor: &Cursor) -> bool {
        self.peek_as::<Stmt>(cursor).unwrap_or(false)
    }

    fn stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        self.parse_as::<Stmt>(cursor)
    }

    fn is_type(&self, cursor: &Cursor) -> bool {
        self.peek_as::<Type>(cursor).unwrap_or(false)
    }

    fn ty(&mut self, cursor: &mut Cursor) -> Result<Type> {
        self.parse_as::<Type>(cursor)
    }

    fn is_vis(&self, cursor: &Cursor) -> bool {
        self.peek_as::<Type>(cursor).unwrap_or(false)
    }

    fn vis(&mut self, cursor: &mut Cursor) -> Result<Visibility> {
        self.parse_as::<Visibility>(cursor)
    }
}
