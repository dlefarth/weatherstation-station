use crate::measurement::Measurement;
use crate::own_config::Config;
use reqwest::blocking::{Client, Response};
use reqwest::Error;

pub fn send_measurements(
    measurements: &Vec<Measurement>,
    config: &Config,
) -> Result<Response, Error> {
    Client::new()
        .post(&config.url)
        .basic_auth(&config.username, Some(&config.password))
        .json(measurements)
        .send()
}
