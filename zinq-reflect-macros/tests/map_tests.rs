#![allow(unused)]

use zinq_reflect::{TypeOf, type_of, value_of};
use zinq_reflect_macros::*;

#[derive(Reflect)]
pub struct Tester(Vec<u16>);

#[test]
pub fn should_reflect_seq() {
    let ty = type_of!(Tester).to_struct();
    let inner = &ty.fields()[0];

    assert_eq!(inner.ty(), &type_of!(Vec<u16>));
}
