// GitHub Portfolio Project: 02_Ownership
// Demonstrates Rust's unique ownership system, move semantics, and borrowing rules.

fn main() {
    // --- PART 1: MOVE SEMANTICS (Taşıma Semantiği) ---
    let s1 = String::from("hello");
    let s2 = s1; // Ownership moved from s1 to s2. s1 is now invalid.
    
    // println!("{}, world!", s1); // Error: value borrowed here after move
    println!("{}, world!", s2); // Works fine!

    // --- PART 2: CLONE (Kopyalama) ---
    let s3 = String::from("hello");
    let s4 = s3.clone(); // Deep copy created. Both s3 and s4 remain valid.
    println!("s3 = {}, s4 = {}", s3, s4);

    // --- PART 3: FUNCTIONS AND OWNERSHIP ---
    let s = String::from("hello");
    take_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here.

    let x = 5;
    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward.

} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn take_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.