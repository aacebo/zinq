#![allow(unused)]

use zinq_reflect::TypeOf;
use zinq_reflect_macros::*;

#[derive(Reflect)]
enum Kind {
    Admin,
    Moderator,
    Basic,
}

#[derive(Reflect)]
struct User {
    pub kind: Kind,
    pub name: String,
    pub password: String,
}

#[derive(Reflect)]
struct Position(f64, f64);

#[test]
pub fn should_reflect_struct() {
    let user = User {
        kind: Kind::Basic,
        name: String::from("dev"),
        password: String::from("test"),
    };

    assert!(user.to_type().is_struct());
    assert_eq!(user.to_type().len(), 3);
    assert!(user.to_type().to_struct().fields()["kind"].ty().is_enum());
}

#[test]
pub fn should_reflect_enum() {
    let kind = Kind::Admin;

    assert!(kind.to_type().is_enum());
    assert_eq!(kind.to_type().len(), 3);
}

#[test]
pub fn should_reflect_tuple_struct() {
    let pos = Position(-500.1, 1034.45);

    assert!(pos.to_type().is_struct());
    assert_eq!(pos.to_type().len(), 2);
    assert!(pos.to_type().to_struct().fields()[0].ty().is_f64());
}
