pub fn run()
{
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Explicit
    let z: i64 = 43554325325423542;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
}
