use zinq_reflect::TypeOf;
use zinq_reflect_macros::*;

#[derive(Reflect)]
struct User {
    #[allow(unused)]
    pub name: String,

    #[allow(unused)]
    pub password: String,
}

#[test]
pub fn should_reflect_struct() {
    let user = User {
        name: String::from("dev"),
        password: String::from("test"),
    };

    assert!(user.to_type().is_struct());
    assert_eq!(user.to_type().to_struct().len(), 2);
}
