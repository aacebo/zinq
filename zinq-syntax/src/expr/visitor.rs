use super::*;

pub trait ExprVisitor {
    fn visit_arithmetic_expr(&mut self, node: &ArithmeticExpr);
    fn visit_assign_expr(&mut self, node: &AssignExpr);
    fn visit_cmp_expr(&mut self, node: &CmpExpr);
    fn visit_is_expr(&mut self, node: &IsExpr);
    fn visit_logical_expr(&mut self, node: &LogicalExpr);

    fn visit_match_expr(&mut self, node: &MatchExpr);
    fn visit_if_expr(&mut self, node: &IfExpr);
    fn visit_range_expr(&mut self, node: &RangeExpr);

    fn visit_call_expr(&mut self, node: &CallExpr);
    fn visit_index_expr(&mut self, node: &IndexExpr);
    fn visit_member_expr(&mut self, node: &MemberExpr);
    fn visit_ref_expr(&mut self, node: &RefExpr);

    fn visit_neg_expr(&mut self, node: &NegExpr);
    fn visit_not_expr(&mut self, node: &NotExpr);

    fn visit_array_expr(&mut self, node: &ArrayExpr);
    fn visit_group_expr(&mut self, node: &GroupExpr);
    fn visit_literal_expr(&mut self, node: &LiteralExpr);
    fn visit_path_expr(&mut self, node: &PathExpr);
    fn visit_struct_expr(&mut self, node: &StructExpr);
    fn visit_tuple_expr(&mut self, node: &TupleExpr);

    fn visit_expr(&mut self, node: &Expr) {
        match node {
            Expr::Arithmetic(v) => self.visit_arithmetic_expr(v),
            Expr::Array(v) => self.visit_array_expr(v),
            Expr::Assign(v) => self.visit_assign_expr(v),
            Expr::Call(v) => self.visit_call_expr(v),
            Expr::Cmp(v) => self.visit_cmp_expr(v),
            Expr::Group(v) => self.visit_group_expr(v),
            Expr::If(v) => self.visit_if_expr(v),
            Expr::Index(v) => self.visit_index_expr(v),
            Expr::Is(v) => self.visit_is_expr(v),
            Expr::Literal(v) => self.visit_literal_expr(v),
            Expr::Logical(v) => self.visit_logical_expr(v),
            Expr::Match(v) => self.visit_match_expr(v),
            Expr::Member(v) => self.visit_member_expr(v),
            Expr::Neg(v) => self.visit_neg_expr(v),
            Expr::Not(v) => self.visit_not_expr(v),
            Expr::Path(v) => self.visit_path_expr(v),
            Expr::Range(v) => self.visit_range_expr(v),
            Expr::Ref(v) => self.visit_ref_expr(v),
            Expr::Struct(v) => self.visit_struct_expr(v),
            Expr::Tuple(v) => self.visit_tuple_expr(v),
        }
    }
}