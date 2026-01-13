#![allow(unused)]

use crate::ty::*;

pub trait TypeVisitor {
    fn visit_type(&mut self, node: &Type) {}
    fn visit_path_type(&mut self, node: &PathType) {}
    fn visit_mut_type(&mut self, node: &MutType) {}
    fn visit_ref_type(&mut self, node: &RefType) {}
    fn visit_slice_type(&mut self, node: &SliceType) {}
    fn visit_tuple_type(&mut self, node: &TupleType) {}
}
