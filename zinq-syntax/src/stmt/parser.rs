use zinq_error::Result;
use zinq_parse::{Cursor, ZinqParser};

use crate::stmt::Stmt;

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
    fn parse_stmt(&mut self, _: &mut Cursor) -> Result<Stmt> {
        todo!()
    }

    fn parse_declare_stmt(&mut self, _: &mut Cursor) -> Result<Stmt> {
        todo!()
    }

    fn parse_if_stmt(&mut self, _: &mut Cursor) -> Result<Stmt> {
        todo!()
    }

    fn parse_return_stmt(&mut self, _: &mut Cursor) -> Result<Stmt> {
        todo!()
    }

    fn parse_var_stmt(&mut self, _: &mut Cursor) -> Result<Stmt> {
        todo!()
    }

    fn parse_struct_stmt(&mut self, _: &mut Cursor) -> Result<Stmt> {
        todo!()
    }

    fn parse_for_stmt(&mut self, _: &mut Cursor) -> Result<Stmt> {
        todo!()
    }

    fn parse_expr_stmt(&mut self, _: &mut Cursor) -> Result<Stmt> {
        todo!()
    }

    fn parse_fn_stmt(&mut self, _: &mut Cursor) -> Result<Stmt> {
        todo!()
    }

    fn parse_block_stmt(&mut self, _: &mut Cursor) -> Result<Stmt> {
        todo!()
    }

    fn parse_use_stmt(&mut self, _: &mut Cursor) -> Result<Stmt> {
        todo!()
    }
}
