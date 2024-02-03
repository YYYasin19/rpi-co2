use serial::unix::TTYPort;
use serial::SerialPort;

use std::io::{Read, Write};
use std::time::Duration;

use mh_z19::{calibrate_zero_point, parse_gas_concentration_ppm, read_gas_concentration};

pub struct Sensor {
    port: TTYPort,
    serial_device: String, // e.g. /dev/ttyAMA0
    device_number: u8,     // e.g. 0x1
}

impl Sensor {
    pub fn new(device: &str) -> Result<Self, serial::Error> {
        let settings: serial::PortSettings = serial::PortSettings {
            baud_rate: serial::BaudRate::Baud9600,
            char_size: serial::CharSize::Bits8,
            parity: serial::Parity::ParityNone,
            stop_bits: serial::StopBits::Stop1,
            flow_control: serial::FlowControl::FlowNone,
        };
        let mut port = serial::open(device)?;
        port.set_timeout(Duration::from_secs(1))?;
        port.configure(&settings)?;
        Ok(Self {
            port,
            serial_device: device.to_string(),
            device_number: 0x1,
        })
    }

    /*
     * This function should send the test command to the sensor
     */
    pub fn check_port(&mut self) -> bool {
        let packet = read_gas_concentration(self.device_number);
        match self.port.write(&packet) {
            Ok(_) => {
                println!("Sent [read_gas_concentration] command.");
                let mut response_vec: Vec<u8> = vec![0; 9];
                match self.port.read(&mut response_vec[..]) {
                    Ok(_) => {
                        println!("Read response: {:?}", response_vec);
                        true
                    }
                    Err(e) => {
                        eprintln!("Failed to read response: {:?}", e);
                        false
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to send command: {:?}", e);
                false
            }
        }
    }

    pub fn clear_buffer(&mut self) {
        let mut buffer: Vec<u8> = vec![0; 9];
        match self.port.read(&mut buffer[..]) {
            Ok(_) => println!("Cleared buffer"),
            Err(e) => eprintln!("Failed to clear buffer: {:?}", e),
        }
    }

    /*
     * Reads the gas concentration in ppm from the sensor
     */
    pub fn read_ppm(&mut self) -> Option<u32> {
        let read_gas_cmd = read_gas_concentration(self.device_number);
        let read_gas_cmd = [0xff, 0x01, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00];
        match self.port.write(&read_gas_cmd) {
            Ok(_) => println!("Sent [read gas concentration] command"),
            Err(e) => eprintln!("Failed to send command: {:?}", e),
        }

        // read response
        let mut response: Vec<u8> = vec![0; 9];
        match self.port.read(&mut response[..]) {
            Ok(_) => {
                let hex_string: Vec<String> =
                    response.iter().map(|b| format!("{:02x}", b)).collect();
                println!("Read: {:?}", hex_string);
                match parse_gas_concentration_ppm(&response) {
                    Ok(ppm) => return Some(ppm),
                    Err(e) => {
                        eprintln!(
                            "Failed to parse response: {:?} for {:?}",
                            e, self.serial_device
                        );
                        return None;
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from port: {:?}", e);
                return None;
            }
        }
    }

    /*
     * Calibrate the sensor to zero.
     * Requires the sensor to be in a clean air environment (400ppm) for at least 20 minutes.
     */
    pub fn calibrate_zero(&mut self) -> Result<(), serial::Error> {
        let packet = calibrate_zero_point(0x1);
        self.port.write(&packet)?;
        let mut response: Vec<u8> = vec![0; 9];
        self.port.read(&mut response[..])?;
        Ok(())
    }
}
