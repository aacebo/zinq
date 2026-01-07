use zinq_error::Result;
use zinq_parse::{Cursor, ZinqParser};

use crate::stmt::{BlockStmt, ExprStmt, FnStmt, LetStmt, Stmt, StructStmt, UseStmt};

pub trait StmtParser {
    fn parse_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_declare_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_if_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_return_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_var_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_struct_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_for_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_expr_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_fn_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_block_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_use_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
}

impl StmtParser for ZinqParser {
    fn parse_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        Ok(self.parse::<Stmt>(cursor)?.into())
    }

    fn parse_declare_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        if self.peek::<StructStmt>(cursor).unwrap_or(false) {
            return self.parse_struct_stmt(cursor);
        } else if self.peek::<FnStmt>(cursor).unwrap_or(false) {
            return self.parse_fn_stmt(cursor);
        } else if self.peek::<LetStmt>(cursor).unwrap_or(false) {
            return self.parse_var_stmt(cursor);
        }

        self.parse_stmt(cursor)
    }

    fn parse_if_stmt(&mut self, _: &mut Cursor) -> Result<Stmt> {
        todo!()
    }

    fn parse_return_stmt(&mut self, _: &mut Cursor) -> Result<Stmt> {
        todo!()
    }

    fn parse_var_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        Ok(self.parse::<LetStmt>(cursor)?.into())
    }

    fn parse_struct_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        Ok(self.parse::<StructStmt>(cursor)?.into())
    }

    fn parse_for_stmt(&mut self, _: &mut Cursor) -> Result<Stmt> {
        todo!()
    }

    fn parse_expr_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        Ok(self.parse::<ExprStmt>(cursor)?.into())
    }

    fn parse_fn_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        Ok(self.parse::<FnStmt>(cursor)?.into())
    }

    fn parse_block_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        Ok(self.parse::<BlockStmt>(cursor)?.into())
    }

    fn parse_use_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        Ok(self.parse::<UseStmt>(cursor)?.into())
    }
}
