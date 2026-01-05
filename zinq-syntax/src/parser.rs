use zinq_error::Result;
use zinq_parse::Cursor;

use crate::{Visibility, expr::Expr, stmt::Stmt, ty::Type};

pub trait SyntaxParser {
    fn is_expr(&self, cursor: &Cursor) -> bool;
    fn expr(&mut self, cursor: &mut Cursor) -> Result<Expr>;

    fn is_stmt(&self, cursor: &Cursor) -> bool;
    fn stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;

    fn is_type(&self, cursor: &Cursor) -> bool;
    fn ty(&mut self, cursor: &mut Cursor) -> Result<Type>;

    fn is_vis(&self, cursor: &Cursor) -> bool;
    fn vis(&mut self, cursor: &mut Cursor) -> Result<Visibility>;
}

impl SyntaxParser for zinq_parse::ZinqParser {
    fn is_expr(&self, cursor: &Cursor) -> bool {
        self.peek::<Expr>(cursor).unwrap_or(false)
    }

    fn expr(&mut self, cursor: &mut Cursor) -> Result<Expr> {
        self.parse::<Expr>(cursor)
    }

    fn is_stmt(&self, cursor: &Cursor) -> bool {
        self.peek::<Stmt>(cursor).unwrap_or(false)
    }

    fn stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        self.parse::<Stmt>(cursor)
    }

    fn is_type(&self, cursor: &Cursor) -> bool {
        self.peek::<Type>(cursor).unwrap_or(false)
    }

    fn ty(&mut self, cursor: &mut Cursor) -> Result<Type> {
        self.parse::<Type>(cursor)
    }

    fn is_vis(&self, cursor: &Cursor) -> bool {
        self.peek::<Type>(cursor).unwrap_or(false)
    }

    fn vis(&mut self, cursor: &mut Cursor) -> Result<Visibility> {
        self.parse::<Visibility>(cursor)
    }
}
