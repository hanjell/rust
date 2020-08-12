pub fn run() {
    let name = "Brad";

    let mut age = 23;

    println!("My name is {} and I'm {}", name,age);

    age = 38;

    println!("My name is {} and I'm {}", name,age);

    // Define constants

    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Multiple vars
    let (my_name, my_age) = ("Brad",37);
    println!("{} is {}", my_name, my_age);
}