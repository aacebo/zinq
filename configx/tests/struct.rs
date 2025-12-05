use configx::Config;

#[derive(Config)]
pub struct ClientConfig {
    id: String,
    secret: String,
}

#[test]
pub fn should_load_struct() {
    let config = ClientConfig {
        id: String::from("test"),
        secret: String::from("test"),
    };
}
