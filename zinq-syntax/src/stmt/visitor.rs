#![allow(unused)]

use super::*;

pub trait StmtVisitor {
    fn visit_stmt(&mut self, node: &Stmt) {}
    fn visit_impl_stmt(&mut self, node: &ImplStmt) {}
    fn visit_block_stmt(&mut self, node: &BlockStmt) {}
    fn visit_enum_stmt(&mut self, node: &EnumStmt) {}
    fn visit_expr_stmt(&mut self, node: &ExprStmt) {}
    fn visit_fn_stmt(&mut self, node: &FnStmt) {}
    fn visit_for_stmt(&mut self, node: &ForStmt) {}
    fn visit_if_stmt(&mut self, node: &IfStmt) {}
    fn visit_let_stmt(&mut self, node: &LetStmt) {}
    fn visit_mod_stmt(&mut self, node: &ModStmt) {}
    fn visit_return_stmt(&mut self, node: &ReturnStmt) {}
    fn visit_struct_stmt(&mut self, node: &StructStmt) {}
    fn visit_use_stmt(&mut self, node: &UseStmt) {}
}
