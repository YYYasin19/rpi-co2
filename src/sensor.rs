use serial::unix::TTYPort;
use serial::SerialPort;

use std::io::{Read, Write};
use std::{thread, time::Duration};

use mh_z19::{calibrate_zero_point, parse_gas_concentration_ppm, read_gas_concentration};

pub struct Sensor {
    port: Option<TTYPort>,
    serial_device: String, // e.g. /dev/ttyAMA0
    device_number: u8,     // e.g. 0x1
}

impl Sensor {
    pub fn new(device: String) -> Result<Self, serial::Error> {
        let settings: serial::PortSettings = serial::PortSettings {
            baud_rate: serial::BaudRate::Baud9600,
            char_size: serial::CharSize::Bits8,
            parity: serial::Parity::ParityNone,
            stop_bits: serial::StopBits::Stop1,
            flow_control: serial::FlowControl::FlowNone,
        };
        let mut port = serial::open(&device)?;
        port.set_timeout(Duration::from_secs(1))?;
        port.configure(&settings)?;
        match port.write(&[0xff, 0x01, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00]) {
            Ok(_) => println!("Sent test command with 8 bytes"),
            Err(e) => eprintln!("Failed to send command: {:?}", e),
        }
        Ok(Self {
            port: Some(port),
            serial_device: device.to_string(),
            device_number: 0x1,
        })
    }

    /*
     * Create a mock sensor for testing. This one does not require the device to be connected.
     */
    #[allow(unused)]
    pub fn new_mock(device: String) -> Result<Self, serial::Error> {
        Ok(Self {
            port: None,
            serial_device: "/dev/ttyAMA0".to_string(),
            device_number: 0x1,
        })
    }

    /*
     * Create a mock sensor for testing. This one does not require the device to be connected.
     */
    pub fn read_ppm_loop_mock(&mut self) {
        loop {
            let ppm = 400 + (rand::random::<u8>() % 100) as i32;
            let now = chrono::Local::now();
            let timestamp = now.format("%Y-%m-%d %H:%M:%S");
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("./values.csv")
                .unwrap();
            writeln!(file, "{},{}", timestamp, ppm).unwrap();
            // println!("{:?}", ppm);
            thread::sleep(Duration::from_secs(1));
        }
    }

    /*
     * Reads the gas concentration in ppm from the sensor
     */
    pub fn read_ppm_loop(&mut self) {
        loop {
            // write command
            let packet = read_gas_concentration(self.device_number);
            if let Some(ref mut port) = self.port {
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
                            Ok(ppm) => {
                                // append to a file `values.csv` with timestamp, ppm
                                let now = chrono::Local::now();
                                let timestamp = now.format("%Y-%m-%d %H:%M:%S");
                                let mut file = std::fs::OpenOptions::new()
                                    .append(true)
                                    .create(true)
                                    .open("./values.csv")
                                    .unwrap();
                                writeln!(file, "{},{}", timestamp, ppm).unwrap();
                                // flush the file
                                // println!("{:?}", ppm);
                            }
                            Err(e) => eprintln!("Failed to parse response: {:?}", e),
                        }
                    }
                    Err(e) => eprintln!(
                        "Failed to read from port: {:?} on {:?}",
                        e, self.serial_device
                    ),
                }
            }

            // sleep a few seconds
            thread::sleep(Duration::from_secs(1));
        }
    }

    /*
     * Calibrate the sensor to zero.
     * Requires the sensor to be in a clean air environment (400ppm) for at least 20 minutes.
     */
    #[allow(dead_code)]
    pub fn calibrate_zero(&mut self) -> Result<(), serial::Error> {
        let packet = calibrate_zero_point(0x1);
        if let Some(port) = &mut self.port {
            port.write(&packet)?;
            let mut response: Vec<u8> = vec![0; 9];
            port.read(&mut response[..])?;
        }
        Ok(())
    }
}
