use configx::Config;

#[derive(Config)]
pub struct ClientConfig {
    pub id: String,
    pub secret: String,
    pub api: ApiConfig,
}

#[derive(Config, Debug)]
pub struct ApiConfig {
    pub url: String,
    pub port: Option<usize>,
}

#[test]
pub fn should_load_struct() {
    let config = ClientConfig {
        id: String::from("test"),
        secret: String::from("test"),
        api: ApiConfig {
            url: String::from("http://google.com"),
            port: Some(3000),
        },
    };
}
