// GitHub Portfolio Project: 05_Port_Scanner
// A functional, simple port scanner using standard library networking.
// Scans ports 1-100 on localhost to find open services.

use std::net::TcpStream;
use std::io::{self, Write};

fn main() {
    let target_ip = "127.0.0.1";
    println!("Starting scan on: {}", target_ip);

    // Scan ports 1 to 100
    for port in 1..100 {
        let address = format!("{}:{}", target_ip, port);

        // Try to connect using TCP. If successful, the port is OPEN.
        // TcpStream::connect returns a Result (Ok or Err).
        match TcpStream::connect(&address) {
            Ok(_) => {
                println!("Port {} is OPEN [✓]", port);
            }
            Err(_) => {
                // Connection failed (refused/timeout), so port is CLOSED.
                // We do nothing to keep output clean.
            }
        }
    }
    println!("Scan complete.");
}