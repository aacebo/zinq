#![allow(unused)]

use zinq_reflect::{Reflect, TypeOf, value_of};
use zinq_reflect_macros::*;

#[reflect(version = 2)]
mod models {
    use zinq_reflect::TypeOf;
    use zinq_reflect_macros::*;

    #[derive(Reflect)]
    pub enum Kind {
        Admin(String),
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
    let kind = models::Kind::Admin("test".to_string());

    assert!(kind.to_type().is_enum());
    assert_eq!(kind.to_type().len(), 3);
    assert!(kind.to_type().to_enum().has_variant("Admin"));
    assert_eq!(kind.to_type().to_enum().variant("Admin").len(), 1);
    assert!(
        kind.to_type().to_enum().variant("Admin").fields()[0]
            .ty()
            .is_string()
    );
}

#[test]
pub fn should_reflect_tuple_struct() {
    let pos = models::Position(-500.1, 1034.45);

    assert!(pos.to_type().is_struct());
    assert_eq!(pos.to_type().len(), 2);
    assert!(pos.to_type().to_struct().fields()[0].ty().is_f64());
}

#[test]
pub fn should_reflect_path() {
    let user = models::User {
        kind: models::Kind::Basic,
        name: String::from("dev"),
        password: String::from("test"),
    };

    assert_eq!(user.to_type().path().to_string(), "derive::models");
}

#[test]
pub fn should_reflect_mod() {
    let ty = models::type_of();

    assert!(ty.is_mod());
    assert_eq!(ty.to_mod().path().name(), "models");
    assert!(ty.to_mod().vis().is_private());
    assert_eq!(ty.to_mod().tys().len(), 3);

    let module = ty.to_mod();

    assert!(module.meta().has("version"));
    assert_eq!(module.meta().get("version").unwrap(), &value_of!(2));

    let one = &module.tys()[0];

    assert!(one.is_enum());
    assert_eq!(one.to_enum().name(), "Kind");

    let two = &module.tys()[1];

    assert!(two.is_struct());
    assert_eq!(two.to_struct().name(), "User");

    let three = &module.tys()[2];

    assert!(three.is_struct());
    assert_eq!(three.to_struct().name(), "Position");
}
