use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use crate::myserial::{DataBits, FlowControl, Parity, SerialPortInfo, StopBits};

use anyhow::Error;
use anyhow::{anyhow, Result};
use bitcore::api as bitcore;
use serialport::SerialPortBuilder;
use std::result::Result::Ok;
use std::thread::JoinHandle;
use std::{thread, time::Duration};

pub struct TerminalController {
    thread_id: u32,
    thread_handle: Option<JoinHandle<()>>,
    thread_transmitter: Option<Box<Sender<String>>>,
    received_data_receiver: Option<Receiver<String>>,
}
impl TerminalController {
    pub fn new(thread_id: u32) -> Self {
        TerminalController {
            thread_id: thread_id,
            thread_handle: None,
            thread_transmitter: None,
            received_data_receiver: None,
        }
    }

    pub fn create_stream(&mut self, sinfo: &SerialPortInfo) {
        // Create stream should always be used in tandem with get_latest_thread_created
        let sinfo_2 = sinfo.clone();

        let thread_closure = move |thread_id: u32,
                                   rx: Receiver<String>,
                                   tx_received: Sender<String>,
                                   sinfo: SerialPortInfo| {
            let thread_controller = TerminalRunner::new(thread_id, rx, tx_received);
            thread_controller.bitcore_action_loop(sinfo);
        };

        let (tx_send, rx_send) = mpsc::channel(); // For sending data to serial port
        let (tx_received, rx_received) = mpsc::channel(); // For receiving data from serial port

        let thread_id = self.thread_id;
        let handle =
            thread::spawn(move || thread_closure(thread_id, rx_send, tx_received, sinfo_2));

        self.thread_transmitter = Some(Box::new(tx_send));
        self.received_data_receiver = Some(rx_received);
        self.thread_handle = Some(handle);

        // print indication that stream has been created
        println!("Stream created!");
        println!("Controller: Transmitter stored, ready to send data");
    }

    pub fn end_stream(&mut self) {
        // Drop the transmitter first to signal the thread to exit
        self.thread_transmitter = None;

        // Don't wait for the thread to join to avoid blocking the UI
        // The thread will exit when it detects the transmitter is disconnected
        if let Some(handle) = self.thread_handle.take() {
            // Spawn a separate task to handle the cleanup without blocking
            std::thread::spawn(move || match handle.join() {
                std::result::Result::Ok(_) => {
                    println!("Thread successfully joined!");
                }
                Err(_) => {
                    println!("Thread failed to join!");
                }
            });
        }

        println!("Stream disconnection initiated");
    }

    pub fn push(&self, data: String) -> Result<()> {
        println!("Controller: push() called with data: '{}'", data);
        let tx = &self.thread_transmitter;
        match tx {
            Some(sender) => {
                println!("Controller: Sending data to thread via channel");
                match sender.send(data.clone()) {
                    std::result::Result::Ok(_) => {
                        println!("Controller: Successfully sent data '{}' to thread", data);
                        return Ok(());
                    }
                    Err(e) => {
                        println!("Controller: Failed to send data to thread: {:?}", e);
                        return Err(anyhow!("Receiver disconnected!"));
                    }
                }
            }
            None => {
                println!("Controller: No transmitter available!");
                Err(anyhow!("Receiver does not exist!"))
            }
        }
    }

    pub fn try_receive_data(&self) -> Option<String> {
        if let Some(receiver) = &self.received_data_receiver {
            match receiver.try_recv() {
                Ok(data) => Some(data),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

struct TerminalRunner {
    thread_id: u32,
    receiver: Receiver<String>,
    received_data_sender: Sender<String>,
}
impl TerminalRunner {
    fn new(
        thread_id: u32,
        receiver: Receiver<String>,
        received_data_sender: Sender<String>,
    ) -> Self {
        TerminalRunner {
            thread_id,
            receiver,
            received_data_sender,
        }
    }

    fn bitcore_action_loop(&self, sinfo: SerialPortInfo) {
        // define shared connection
        let connection: bitcore::SharedConnection = Arc::new(Mutex::new(None));

        let sinfo_b: SerialPortBuilder = sinfo.into();
        // Fix timeout - use a reasonable timeout instead of u64::MAX
        let sinfo_b = sinfo_b.timeout(Duration::from_millis(100));

        // open connection
        println!(
            "Thread {}: Attempting to connect to serial port",
            self.thread_id
        );
        match bitcore::connect(&connection, sinfo_b) {
            Ok(_) => {
                println!(
                    "Thread {}: Successfully connected to serial port",
                    self.thread_id
                );
            }
            Err(e) => {
                println!(
                    "Thread {}: Failed to connect to serial port: {:?}",
                    self.thread_id, e
                );
                return;
            }
        }

        println!(
            "Starting serial communication loop for thread {}",
            self.thread_id
        );

        // Small delay to ensure controller is ready
        std::thread::sleep(Duration::from_millis(100));
        println!("Thread {}: Ready to receive data", self.thread_id);

        let mut loop_count = 0;
        let mut last_status_report = std::time::Instant::now();

        loop {
            loop_count += 1;

            // Report status every 5 seconds to ensure thread is alive
            if last_status_report.elapsed() >= Duration::from_secs(5) {
                println!(
                    "Thread {}: Still running (loop {})",
                    self.thread_id, loop_count
                );
                last_status_report = std::time::Instant::now();
            }

            if loop_count == 1 {
                println!("Thread {}: First loop iteration started", self.thread_id);
            }

            // Check for data to send
            let received_data = self.receiver.try_recv();
            if loop_count <= 10 {
                println!(
                    "Thread {}: try_recv result: {:?}",
                    self.thread_id, received_data
                );
            }

            match received_data {
                std::result::Result::Ok(data) => {
                    println!(
                        "Thread {}: Received data to send: '{}'",
                        self.thread_id, data
                    );
                    // Send data without adding extra characters for now
                    match bitcore::write(&connection, data.as_bytes(), 1) {
                        Ok(_) => {
                            println!(
                                "Thread {}: Successfully wrote data to serial port",
                                self.thread_id
                            );
                        }
                        Err(e) => {
                            println!(
                                "Thread {}: Failed to write data to serial port: {:?}",
                                self.thread_id, e
                            );
                        }
                    }
                }
                Err(e) => match e {
                    std::sync::mpsc::TryRecvError::Empty => {
                        // No data to send, continue to read
                    }
                    std::sync::mpsc::TryRecvError::Disconnected => {
                        println!(
                            "Thread {}: Receiver disconnected, terminating",
                            self.thread_id
                        );
                        break;
                    }
                },
            }

            // Read incoming data
            if loop_count <= 5 {
                println!("Thread {}: About to read from serial port", self.thread_id);
            }

            // Try to catch any panics from the bitcore::read call
            let read_data = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut read_buf = vec![0; 1024]; // Increased buffer size
                let result = bitcore::read(&connection, &mut read_buf, Duration::from_millis(10)); // Shorter timeout for non-blocking behavior
                (result, read_buf)
            }));

            if loop_count <= 5 {
                println!("Thread {}: Read operation completed", self.thread_id);
            }

            let (read_result, read_buf) = match read_data {
                Ok((result, buf)) => (result, buf),
                Err(e) => {
                    println!(
                        "Thread {}: Read operation panicked: {:?}",
                        self.thread_id, e
                    );
                    // Small delay before continuing to prevent rapid panic loops
                    std::thread::sleep(Duration::from_millis(10));
                    continue; // Skip this iteration and try again
                }
            };

            match read_result {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        let data_str = String::from_utf8_lossy(&read_buf[..bytes_read]);
                        println!(
                            "Thread {}: Raw received {} bytes: {:?}",
                            self.thread_id, bytes_read, data_str
                        );

                        // Send all received data, even if it contains control characters
                        let message = format!("Received: {}", data_str);
                        match self.received_data_sender.send(message) {
                            Ok(_) => {
                                println!(
                                    "Thread {}: Successfully sent received data to UI",
                                    self.thread_id
                                );
                            }
                            Err(e) => {
                                println!(
                                    "Thread {}: Failed to send received data to UI: {:?}",
                                    self.thread_id, e
                                );
                                // If we can't send to UI, the receiver might be disconnected
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    // Log read errors occasionally for debugging
                    if loop_count <= 10 || loop_count % 1000 == 0 {
                        println!(
                            "Thread {}: Read error (normal timeout): {:?}",
                            self.thread_id, e
                        );
                    }
                }
            }

            // Small delay to prevent busy waiting
            std::thread::sleep(Duration::from_millis(1));

            if loop_count <= 10 {
                println!(
                    "Thread {}: End of loop iteration {}",
                    self.thread_id, loop_count
                );
            }
        }

        // close connection
        println!("Thread {}: Disconnecting from serial port", self.thread_id);
        match bitcore::disconnect(&connection) {
            Ok(_) => {
                println!("Thread {}: Successfully disconnected", self.thread_id);
            }
            Err(e) => {
                println!("Thread {}: Failed to disconnect: {:?}", self.thread_id, e);
            }
        }
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
