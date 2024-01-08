mod sensor;
use sensor::Sensor;
extern crate serial;
use mh_z19::{parse_gas_concentration_ppm, read_gas_concentration};
use serial::core::{BaudRate, CharSize, FlowControl, Parity, PortSettings, StopBits};
use serial::prelude::*;
use serial::unix::TTYPort;
use std::env;
use std::io::{Read, Write};
use std::{thread, time::Duration};

fn run_old() -> Result<(), serial::Error> {
    // read from environment variable CO2_DEVICE
    let device = env::var("CO2_DEVICE").unwrap_or("/dev/ttyAMA0".to_string());
    println!("Reading from device: {}", device);

    let settings: PortSettings = PortSettings {
        baud_rate: BaudRate::Baud9600,
        char_size: CharSize::Bits8,
        parity: Parity::ParityNone,
        stop_bits: StopBits::Stop1,
        flow_control: FlowControl::FlowNone,
    };
    let mut port = serial::open(&device)?;
    port.set_timeout(Duration::from_secs(1))?;
    port.configure(&settings)?;
    match port.write(&[0xff, 0x01, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00]) {
        Ok(_) => println!("Sent test command with 8 bytes"),
        Err(e) => eprintln!("Failed to send command: {:?}", e),
    }

    // read from port and write to stdout
    println!("Reading from port");

    loop {
        // write command
        let packet = read_gas_concentration(0x1);
        match port.write(&packet) {
            Ok(_) => println!("Sent [read gas concentration] command"),
            Err(e) => eprintln!("Failed to send command: {:?}", e),
        }

        // read response
        let mut response: Vec<u8> = vec![0; 9];
        match port.read(&mut response[..]) {
            Ok(t) => {
                println!("Read {} bytes", t);
                let hex_string: Vec<String> =
                    response.iter().map(|b| format!("{:02x}", b)).collect();
                println!("Read: {:?}", hex_string);
                match parse_gas_concentration_ppm(&response) {
                    Ok(ppm) => println!("CO2: {} ppm", ppm),
                    Err(e) => eprintln!("Failed to parse response: {:?}", e),
                }
            }
            Err(e) => eprintln!("Failed to read from port: {:?}", e),
        }

        // sleep a few seconds
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    if env::var("RUN_OLD").unwrap_or("false".to_string()) == "true" {
        println!("Running old code");
        run_old().unwrap();
        return;
    }

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

    for _ in 0..5 {
        match sensor.read_ppm() {
            Some(ppm) => println!("CO2: {} ppm", ppm),
            None => println!("Failed to read CO2"),
        }
        thread::sleep(Duration::from_secs(1));
    }
}
