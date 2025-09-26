#![allow(unused)]

use zinq_reflect::{TypeOf, type_of, value_of};
use zinq_reflect_macros::*;

#[reflect(a = "b")]
trait Hello {
    fn world(&self, a: u8) -> bool;
}

#[test]
pub fn should_reflect_trait() {
    let ty = type_of!(dyn Hello);

    assert!(ty.is_trait());
    assert_eq!(ty.len(), 1);
    assert!(ty.to_trait().has("world"));
    assert_eq!(ty.to_trait().get("world").unwrap().params().len(), 2);
    assert!(ty.to_trait().get("world").unwrap().has_param("self"));
    assert!(ty.to_trait().get("world").unwrap().has_param("a"));
    assert!(ty.to_trait().meta().has("a"));
    assert_eq!(ty.to_trait().meta().get("a").unwrap(), &value_of!("b"));
}
