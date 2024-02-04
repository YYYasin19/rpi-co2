#[path = "../sensor.rs"]
mod sensor;
use sensor::Sensor;
extern crate serial;
use std::env;

fn main() {
    // read from environment variable CO2_DEVICE
    let device = env::var("CO2_DEVICE").unwrap_or("/dev/ttyAMA0".to_string());

    let mut sensor = Sensor::new(device).unwrap();
    sensor.read_ppm_loop();
}
