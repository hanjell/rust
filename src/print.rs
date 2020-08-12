pub fn run()
{
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formating
    println!("String {} argument <{}>",1, "Literal");

    // Positional arguments
    println!("{} is from {1} and {} likes to {2}","Brad","Mass","code");

    //Named
    println!("{name} likes to play {activity}",name = "Brad",activity = "code");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}