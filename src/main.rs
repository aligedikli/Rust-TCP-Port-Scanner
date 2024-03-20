use std::io;
use std::net::TcpStream;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;

struct PortScanner {
    ip: String,
    ports: Vec<u16>,
}

trait Scan {
    fn scan(&self) -> Vec<u16>;
}

impl PortScanner {
    fn new(ip: &str, start_port: u16, end_port: u16) -> Self {
        PortScanner {
            ip: ip.to_string(),
            ports: (start_port..=end_port).collect(),
        }
    }
}

impl Scan for PortScanner {
    fn scan(&self) -> Vec<u16> {
        let num_threads = 100;
        let ports_per_thread = self.ports.len() / num_threads;
        let ports = Arc::new(self.ports.clone());
        let open_ports = Arc::new(Mutex::new(Vec::new()));

        let mut threads = Vec::new();

        for i in 0..num_threads {
            let ports = Arc::clone(&ports);
            let open_ports = Arc::clone(&open_ports);
            let start = i * ports_per_thread;
            let end = if i == num_threads - 1 {
                self.ports.len()
            } else {
                (i + 1) * ports_per_thread
            };

            let ip = self.ip.clone(); // Moved inside the loop to clone for each thread
            let thread = thread::spawn(move || {
                for port in ports[start..end].iter() {
                    let address = format!("{}:{}", ip, port);
                    if TcpStream::connect(&address).is_ok() {
                        let mut open_ports = open_ports.lock().unwrap();
                        open_ports.push(*port);
                    }
                }
            });

            threads.push(thread);
        }

        for thread in threads {
            let _ = thread.join();
        }

        let open_ports = Arc::try_unwrap(open_ports).unwrap().into_inner().unwrap();
        open_ports
    }
}
 // Author: https://github.com/aligedikli 
fn main() {
    println!("Enter the IP address to scan:");
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).expect("Failed to read line");
    let ip = ip.trim(); // Trim newline or whitespace

    let scanner = PortScanner::new(ip, 1, 1024); // IP and Port range to scan
    let open_ports = scanner.scan();

    println!("Open ports:");
    for port in open_ports {
        println!("{}", port);
    }
}
