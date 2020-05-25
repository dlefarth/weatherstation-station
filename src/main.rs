extern crate config;

use crate::own_config::Config;
use crate::measurement::Measurement;

pub mod measurement;
pub mod own_config;
mod queue;
mod sensor_adapter;
mod server_adapter;

fn main() {
    let config = own_config::load();
    
    let measurements = measure_and_get_queue();

    send_to_server_or_queue(&measurements, &config);
}

fn measure_and_get_queue() -> Vec<Measurement> {
    let mut measurements = Vec::new();
    measurements.push(sensor_adapter::measure());
    println!("measured {:?}", measurements[0]);
    let queue = queue::get().unwrap();
    println!("queue cotains {:?}", queue);
    measurements.extend(queue);
    measurements
}

fn send_to_server_or_queue(measurements: &Vec<Measurement>, config: &Config) {
    let http_result = server_adapter::send_measurements(&measurements, &config);
    println!("http result: {:?}", &http_result);
    if http_result.is_ok() && http_result.unwrap().status().is_success() {
        println!("sent successful");
        queue::clear().ok();
    } else {
        println!("not successful sent, write value to queue");
        queue::write(&measurements).expect("failed to write queue");
    }
}
