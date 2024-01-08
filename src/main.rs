mod sensor;
use sensor::Sensor;

use std::env;

fn main() {
    // read from environment variable CO2_DEVICE
    let device = env::var("CO2_DEVICE").unwrap_or("/dev/ttyAMA0".to_string());

    let mut sensor = Sensor::new(&device).unwrap();
    match sensor.check_port() {
        true => println!("Port is open 📶"),
        false => {
            println!("Port is closed. Quittung!");
            return;
        }
    }
    sensor.read_ppm();
}
