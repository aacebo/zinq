#![allow(unused)]

use zinq_reflect::TypeOf;
use zinq_reflect_macros::*;

#[reflect]
fn add(a: i8, b: i8) -> i32 {
    return (a + b) as i32;
}

#[test]
pub fn should_reflect_method() {}
