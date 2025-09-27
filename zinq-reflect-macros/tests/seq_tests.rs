#![allow(unused)]

use zinq_reflect::{TypeOf, type_of, value_of};
use zinq_reflect_macros::*;

#[reflect(a = "b")]
trait Hello<T = String> {
    fn world(&self, a: u8) -> Vec<bool>;
}

#[test]
pub fn should_reflect_seq() {
    let ty = type_of!(dyn Hello).to_trait();
    let world = ty.methods().first().unwrap();

    assert_eq!(world.return_type(), &type_of!(Vec<bool>));
}
