mod sensor;
use sensor::Sensor;
extern crate serial;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mock_mode = args.contains(&"--mock".to_string());

    // read from environment variable CO2_DEVICE
    let device = env::var("CO2_DEVICE").unwrap_or("/dev/ttyAMA0".to_string());

    if mock_mode {
        println!("Running in mock mode");
        let mut sensor = Sensor::new_mock(device).unwrap();
        sensor.read_ppm_loop_mock();
    } else {
        println!("Reading from device: {}", device);
        let mut sensor = Sensor::new(device).unwrap();
        sensor.read_ppm_loop();
    }
}
