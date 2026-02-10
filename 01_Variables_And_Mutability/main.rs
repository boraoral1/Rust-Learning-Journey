// GitHub Portfolio Project: 01_Variables_And_Mutability
// Demonstrates the fundamental concepts of variables, mutability, and shadowing in Rust.

fn main() {
    // 1. Immutable Variable (Değiştirilemez Değişken)
    // By default, variables in Rust are immutable. This provides safety.
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // This would cause a compile-time error!

    // 2. Mutable Variable (Değiştirilebilir Değişken)
    // We use the 'mut' keyword to make a variable mutable.
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 20; // This is allowed because of 'mut'
    println!("The value of y changed to: {}", y);

    // 3. Shadowing (Gölgeleme)
    // We can declare a new variable with the same name as a previous variable.
    let z = 5;
    let z = z + 1; // z is now 6
    {
        let z = z * 2; // z is now 12 (only inside this scope)
        println!("The value of z in the inner scope is: {}", z);
    }
    println!("The value of z in the outer scope is: {}", z); // z is still 6 here

    // 4. Constants (Sabitler)
    // Constants are always immutable and must be annotated.
    const MAX_POINTS: u32 = 100_000;
    println!("Maximum points allowed: {}", MAX_POINTS);
}