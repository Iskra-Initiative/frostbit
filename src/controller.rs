use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use crate::myserial::{DataBits, FlowControl, Parity, SerialPortInfo, StopBits};

use anyhow::Error;
use anyhow::{anyhow, Result};
use bitcore::api as bitcore;
use serialport::{SerialPortBuilder, SerialPortInfo as SInfo};
use std::result::Result::Ok;
use std::thread::JoinHandle;
use std::{io, u64};
use std::{thread, time::Duration};

pub struct TerminalController {
    thread_id: u32,
    thread_handle: Option<JoinHandle<()>>,
    thread_transmitter: Option<Box<Sender<String>>>,
}
impl TerminalController {
    pub fn new(thread_id: u32) -> Self {
        TerminalController {
            thread_id: thread_id,
            thread_handle: None,
            thread_transmitter: None,
        }
    }

    pub fn create_stream(&mut self, sinfo: &SerialPortInfo) {
        // Create stream should always be used in tandem with get_latest_thread_created
        let sinfo_2 = sinfo.clone();

        let thread_closure = move |thread_id: u32, rx: Receiver<String>, sinfo: SerialPortInfo| {
            let thread_controller = TerminalRunner::new(thread_id, rx);
            thread_controller.bitcore_action_loop(sinfo);
        };

        let (tx, rx) = mpsc::channel();
        let thread_id = self.thread_id;
        let handle = thread::spawn(move || thread_closure(thread_id, rx, sinfo_2));
        self.thread_transmitter = Some(Box::new(tx));
        self.thread_handle = Some(handle);

        // print indication that stream has been created
        println!("Stream created!");
    }

    pub fn end_stream(&mut self) {
        self.thread_transmitter = None;
        let handle = self.thread_handle.take();
        match handle {
            Some(handle) => match handle.join() {
                std::result::Result::Ok(_) => {
                    println!("Thread successfully joined!");
                }
                Err(_) => {
                    println!("Thread failed to join!");
                }
            },
            None => {
                println!("Controller: end_stream -> Handle does not exist. This is a logical Error. Ignoring for stability.");
            }
        };
        self.thread_handle = None;
        // Remove transmitter
    }

    pub fn push(&self, data: String) -> Result<()> {
        let tx = &self.thread_transmitter;
        match tx {
            Some(sender) => match sender.send(data) {
                std::result::Result::Ok(_) => return Ok(()),
                Err(_) => return Err(anyhow!("Receiver disconnected!")),
            },
            None => Err(anyhow!("Receiver does not exist!")),
        }
    }
}

struct TerminalRunner {
    thread_id: u32,
    receiver: Receiver<String>,
}
impl TerminalRunner {
    fn new(thread_id: u32, receiver: Receiver<String>) -> Self {
        TerminalRunner {
            thread_id,
            receiver,
        }
    }

    fn bitcore_action_loop(&self, sinfo: SerialPortInfo) {
        // define shared connection
        let connection: bitcore::SharedConnection = Arc::new(Mutex::new(None));

        let sinfo_b: SerialPortBuilder = sinfo.into();
        let sinfo_b = sinfo_b.timeout(Duration::from_secs(u64::MAX));

        // open connection
        bitcore::connect(&connection, sinfo_b).expect("Failed to connect to serial port");

        loop {
            let received_data = self.receiver.try_recv();

            match received_data {
                std::result::Result::Ok(data) => {
                    // add carriage return and newline to data
                    let data = format!("{}\r\n", data);

                    bitcore::write(&connection, data.as_bytes(), 1)
                        .expect("Failed to write data to serial port");
                }
                Err(e) => match e {
                    std::sync::mpsc::TryRecvError::Empty => {}
                    std::sync::mpsc::TryRecvError::Disconnected => {
                        println!("Terminating Thread {:?}", self.thread_id);
                        break;
                    }
                },
            }

            // read data
            let mut read_buf = vec![0; 64];
            let read_data = bitcore::read(&connection, &mut read_buf, Duration::from_millis(1));

            match read_data {
                Ok(data) => {
                    let data = String::from_utf8_lossy(&read_buf[..data]);
                    println!("Received data: {}", data);
                    println!("Thread Id: {:?}: Data: {:?}", self.thread_id, data);
                }
                Err(e) => {
                    // no data
                }
            }
        }

        // close connection
        bitcore::disconnect(&connection).expect("Failed to disconnect from serial port");
    }
}

pub fn list_available_ports() -> Result<Vec<SerialPortInfo>, Error> {
    let ports = serialport::available_ports()?;
    let mut serial_ports = Vec::new();

    for port in ports {
        let serial_port = SerialPortInfo::new(
            port.port_name,
            9600,
            DataBits::from(DataBits::Eight),
            Parity::from(Parity::None),
            StopBits::from(StopBits::One),
            FlowControl::from(FlowControl::None),
        );

        serial_ports.push(serial_port);
    }

    Ok(serial_ports)
}
