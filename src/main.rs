mod sensor;
use sensor::Sensor;
extern crate serial;
use mh_z19::{parse_gas_concentration_ppm, read_gas_concentration};
use serial::core::{BaudRate, CharSize, FlowControl, Parity, PortSettings, StopBits};
use serial::prelude::*;
use std::env;
use std::io::{Read, Write};
use std::{thread, time::Duration};

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
