// GitHub Portfolio Project: 03_Structs_And_Enums
// Demonstrates custom types: Structs (grouping data) and Enums (listing variants).

// 1. Defining a Struct (Bir Yapı Tanımlama)
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 2. Defining an Enum (Bir Numaralandırma Tanımlama)
// Enums can hold data directly inside their variants.
enum IpAddrKind {
    V4(u8, u8, u8, u8), // IPv4 holds four 8-bit numbers
    V6(String),         // IPv6 holds a String
}

fn main() {
    // --- PART 1: USING STRUCTS ---
    // Creating an instance (Örnek oluşturma)
    let user1 = User {
        email: String::from("student@toros.edu.tr"),
        username: String::from("rust_learner"),
        active: true,
        sign_in_count: 1,
    };

    println!("User Info: {} ({})", user1.username, user1.email);

    // --- PART 2: USING ENUMS AND MATCH ---
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    process_ip(localhost);
    process_ip(loopback);
}

fn process_ip(ip: IpAddrKind) {
    // Pattern Matching to handle different Enum variants
    match ip {
        IpAddrKind::V4(a, b, c, d) => {
            println!("Detected IPv4: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddrKind::V6(addr) => {
            println!("Detected IPv6: {}", addr);
        }
    }
}