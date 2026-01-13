use super::*;

pub trait PatternVisitor {
    #![allow(unused)]

    fn visit_pattern(&mut self, node: &Pattern) {}
    fn visit_group_pattern(&mut self, node: &GroupPattern) {}
    fn visit_literal_pattern(&mut self, node: &LiteralPattern) {}
    fn visit_or_pattern(&mut self, node: &OrPattern) {}
    fn visit_path_pattern(&mut self, node: &PathPattern) {}
    fn visit_ref_pattern(&mut self, node: &RefPattern) {}
    fn visit_spread_pattern(&mut self, node: &SpreadPattern) {}
    fn visit_struct_pattern(&mut self, node: &StructPattern) {}
    fn visit_tuple_pattern(&mut self, node: &TuplePattern) {}
    fn visit_wild_pattern(&mut self, node: &WildPattern) {}
}
