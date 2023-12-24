extern crate serial;
use serial::core::{BaudRate, CharSize, FlowControl, Parity, PortSettings, StopBits};
use serial::prelude::*;
use serial::unix::TTYPort;
use std::io::{Read, Write};

use std::{thread, time::Duration};

use mh_z19::{parse_gas_concentration_ppm, read_gas_concentration};

fn build_port(device: &str) -> Result<TTYPort, serial::Error> {
    let settings: PortSettings = PortSettings {
        baud_rate: BaudRate::Baud115200,
        char_size: CharSize::Bits8,
        parity: Parity::ParityNone,
        stop_bits: StopBits::Stop1,
        flow_control: FlowControl::FlowNone,
    };
    let mut port = serial::open(device)?;
    port.set_timeout(Duration::from_secs(1))?;
    port.configure(&settings)?;
    Ok(port)
}

fn check_port(serial: &mut TTYPort) {
    match serial.write(&[0xff, 0x01, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00]) {
        Ok(_) => println!("Sent test command with 8 bytes"),
        Err(e) => eprintln!("Failed to send command: {:?}", e),
    }
}

fn main() {
    const DEVICE: &str = "/dev/ttys028";
    let mut serial = build_port(DEVICE).unwrap();
    check_port(&mut serial);

    // read from port and write to stdout
    println!("Reading from port");

    loop {
        // write command
        let packet = read_gas_concentration(0x1);
        match serial.write(&packet) {
            Ok(_) => println!("Sent [read gas concentration] command"),
            Err(e) => eprintln!("Failed to send command: {:?}", e),
        }

        // read response
        let mut response: Vec<u8> = vec![0; 9];
        match serial.read(&mut response[..]) {
            Ok(t) => {
                println!("Read {} bytes", t);
                println!("Read: {:?}", response);
                match parse_gas_concentration_ppm(&response) {
                    Ok(ppm) => println!("CO2: {} ppm", ppm),
                    Err(e) => eprintln!("Failed to parse response: {:?}", e),
                }
            }
            Err(e) => eprintln!("Failed to read from port: {:?}", e),
        }

        // sleep a few seconds
        thread::sleep(Duration::from_secs(5));
    }
}
