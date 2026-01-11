use zinq_error::Result;
use zinq_parse::{Cursor, ZinqParser};

use crate::stmt::{
    BlockStmt, EnumStmt, ExprStmt, FnStmt, ForStmt, IfStmt, ImplStmt, LetStmt, ReturnStmt, Stmt,
    StructStmt, UseStmt,
};

pub trait StmtParser {
    fn parse_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_if_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_return_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_var_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_struct_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_enum_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_impl_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_for_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_expr_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_fn_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_block_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
    fn parse_use_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt>;
}

impl StmtParser for ZinqParser {
    fn parse_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        if self.peek::<StructStmt>(cursor).unwrap_or(false) {
            return self.parse_struct_stmt(cursor);
        } else if self.peek::<EnumStmt>(cursor).unwrap_or(false) {
            return self.parse_enum_stmt(cursor);
        } else if self.peek::<FnStmt>(cursor).unwrap_or(false) {
            return self.parse_fn_stmt(cursor);
        } else if self.peek::<LetStmt>(cursor).unwrap_or(false) {
            return self.parse_var_stmt(cursor);
        } else if self.peek::<UseStmt>(cursor).unwrap_or(false) {
            return self.parse_use_stmt(cursor);
        } else if self.peek::<ImplStmt>(cursor).unwrap_or(false) {
            return self.parse_impl_stmt(cursor);
        } else if self.peek::<ForStmt>(cursor).unwrap_or(false) {
            return self.parse_for_stmt(cursor);
        } else if self.peek::<IfStmt>(cursor).unwrap_or(false) {
            return self.parse_if_stmt(cursor);
        } else if self.peek::<ReturnStmt>(cursor).unwrap_or(false) {
            return self.parse_return_stmt(cursor);
        } else if self.peek::<BlockStmt>(cursor).unwrap_or(false) {
            return self.parse_block_stmt(cursor);
        }

        self.parse_expr_stmt(cursor)
    }

    fn parse_if_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        Ok(self.parse::<IfStmt>(cursor)?.into())
    }

    fn parse_return_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        Ok(self.parse::<ReturnStmt>(cursor)?.into())
    }

    fn parse_var_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        Ok(self.parse::<LetStmt>(cursor)?.into())
    }

    fn parse_struct_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        Ok(self.parse::<StructStmt>(cursor)?.into())
    }

    fn parse_enum_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        Ok(self.parse::<EnumStmt>(cursor)?.into())
    }

    fn parse_impl_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        Ok(self.parse::<ImplStmt>(cursor)?.into())
    }

    fn parse_for_stmt(&mut self, cursor: &mut Cursor) -> Result<Stmt> {
        Ok(self.parse::<ForStmt>(cursor)?.into())
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
