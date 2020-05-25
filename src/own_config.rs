use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub url: String,
    pub username: String,
    pub password: String,
}

pub fn load() -> Config {
    let mut config = config::Config::default();
    
    config
        .merge(config::File::with_name("config.toml")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();

    config.try_into::<Config>().unwrap()
}