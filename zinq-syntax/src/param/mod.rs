mod fn_param;
mod self_param;
mod type_param;

pub use fn_param::*;
pub use self_param::*;
pub use type_param::*;

pub trait ParamVisitor {
    #![allow(unused)]

    fn visit_fn_param(&mut self, node: &FnParam) {}
    fn visit_self_param(&mut self, node: &SelfParam) {}
    fn visit_type_param(&mut self, node: &TypeParam) {}
}
