// GitHub Portfolio Project: 04_Result_And_Error_Handling
// Demonstrates Rust's powerful error handling with `Result<T, E>` and `match`.

enum ConnectionError {
    Timeout,
    Refused,
}

// 1. A function that returns a Result (Ok or Err)
// Simulate trying to connect to a server.
fn connect(ip: &str) -> Result<String, ConnectionError> {
    if ip == "192.168.1.1" {
        Ok(String::from("Connected successfully!"))
    } else {
        Err(ConnectionError::Timeout)
    }
}

fn main() {
    let target = "10.0.0.5";

    // 2. Handling the Result with 'match'
    // Rust forces us to handle both success (Ok) and failure (Err).
    match connect(target) {
        Ok(msg) => println!("Success: {}", msg),
        Err(error) => match error {
            ConnectionError::Timeout => println!("Error: Connection timed out."),
            ConnectionError::Refused => println!("Error: Connection refused."),
        },
    }
}