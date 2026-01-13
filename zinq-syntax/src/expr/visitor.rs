#![allow(unused)]

use super::*;

pub trait ExprVisitor {
    fn visit_expr(&mut self, node: &Expr) {}

    fn visit_arithmetic_expr(&mut self, node: &ArithmeticExpr) {}
    fn visit_assign_expr(&mut self, node: &AssignExpr) {}
    fn visit_cmp_expr(&mut self, node: &CmpExpr) {}
    fn visit_is_expr(&mut self, node: &IsExpr) {}
    fn visit_logical_expr(&mut self, node: &LogicalExpr) {}

    fn visit_match_expr(&mut self, node: &MatchExpr) {}
    fn visit_if_expr(&mut self, node: &IfExpr) {}
    fn visit_range_expr(&mut self, node: &RangeExpr) {}

    fn visit_call_expr(&mut self, node: &CallExpr) {}
    fn visit_index_expr(&mut self, node: &IndexExpr) {}
    fn visit_member_expr(&mut self, node: &MemberExpr) {}
    fn visit_ref_expr(&mut self, node: &RefExpr) {}

    fn visit_neg_expr(&mut self, node: &NegExpr) {}
    fn visit_not_expr(&mut self, node: &NotExpr) {}

    fn visit_array_expr(&mut self, node: &ArrayExpr) {}
    fn visit_group_expr(&mut self, node: &GroupExpr) {}
    fn visit_literal_expr(&mut self, node: &LiteralExpr) {}
    fn visit_path_expr(&mut self, node: &PathExpr) {}
    fn visit_struct_expr(&mut self, node: &StructExpr) {}
    fn visit_tuple_expr(&mut self, node: &TupleExpr) {}
}
