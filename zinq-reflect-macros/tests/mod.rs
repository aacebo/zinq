use zinq_reflect::TypeOf;
use zinq_reflect_macros::*;

#[derive(Reflect)]
enum Kind {
    #[allow(unused)]
    Admin,

    #[allow(unused)]
    Moderator,

    #[allow(unused)]
    Basic,
}

#[derive(Reflect)]
struct User {
    #[allow(unused)]
    pub kind: Kind,

    #[allow(unused)]
    pub name: String,

    #[allow(unused)]
    pub password: String,
}

#[test]
pub fn should_reflect_struct() {
    let user = User {
        kind: Kind::Basic,
        name: String::from("dev"),
        password: String::from("test"),
    };

    assert!(user.to_type().is_struct());
    assert_eq!(user.to_type().to_struct().len(), 3);
    assert!(
        user.to_type()
            .to_struct()
            .member("kind")
            .to_field()
            .ty()
            .is_enum()
    );
}

#[test]
pub fn should_reflect_enum() {
    let kind = Kind::Admin;

    assert!(kind.to_type().is_enum());
    assert_eq!(kind.to_type().len(), 3);
}
