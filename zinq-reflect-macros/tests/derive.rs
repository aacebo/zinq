#![allow(unused)]

use zinq_reflect::{Reflect, TypeOf};

mod models {
    use zinq_reflect_macros::*;

    #[derive(Reflect)]
    pub enum Kind {
        Admin,
        Moderator,
        Basic,
    }

    #[derive(Reflect)]
    #[reflect(name = "alex")]
    pub struct User {
        #[reflect(hello = "world")]
        pub kind: Kind,
        pub name: String,
        pub password: String,
    }

    #[derive(Reflect)]
    pub struct Position(pub f64, pub f64);
}

#[test]
pub fn should_reflect_struct() {
    let user = models::User {
        kind: models::Kind::Basic,
        name: String::from("dev"),
        password: String::from("test"),
    };

    assert!(user.to_type().is_struct());
    assert_eq!(user.to_type().len(), 3);
    assert!(user.to_type().to_struct().fields()["kind"].ty().is_enum());
    assert_eq!(user.to_type().to_struct().meta().len(), 1);
    assert!(user.to_type().to_struct().meta().has("name"));
    assert_eq!(
        user.to_type().to_struct().meta().get("name").unwrap(),
        &"alex".reflect()
    );
}

#[test]
pub fn should_reflect_field() {
    let user = models::User {
        kind: models::Kind::Basic,
        name: String::from("dev"),
        password: String::from("test"),
    };

    assert!(user.to_type().is_struct());

    let field = user.to_type().to_struct().fields()["kind"].clone();

    assert!(field.ty().is_enum());
    assert_eq!(field.meta().len(), 1);
    assert!(field.meta().has("hello"));
    assert_eq!(field.meta().get("hello").unwrap(), &"world".reflect());
}

#[test]
pub fn should_reflect_enum() {
    let kind = models::Kind::Admin;

    assert!(kind.to_type().is_enum());
    assert_eq!(kind.to_type().len(), 3);
}

#[test]
pub fn should_reflect_tuple_struct() {
    let pos = models::Position(-500.1, 1034.45);

    assert!(pos.to_type().is_struct());
    assert_eq!(pos.to_type().len(), 2);
    assert!(pos.to_type().to_struct().fields()[0].ty().is_f64());
}

#[test]
pub fn should_reflect_module() {
    let user = models::User {
        kind: models::Kind::Basic,
        name: String::from("dev"),
        password: String::from("test"),
    };

    assert_eq!(user.to_type().path().to_string(), "derive::models");
}
