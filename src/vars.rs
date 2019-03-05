// Variables hold primitive data or reference to data
// Variables are immutable by default (so they cannot be reassigned)
// Rust is block-scoped language (meaning that variables are defined only in current scope)

pub fn run() {
    let name = "Brad";
    println!("Name is: {}", name);

    // Now we have a variable that we want to change over time:
    let mut age = 37;
    age = 38;

    // Let's define a constant
    // While defining a constant we have to specify the type of the variable
    const ID: i32 = 001; 

    // Lets define multiple variables at the same time:
    let (my_name, my_age) = ("Ivan", 13);
    println!("My name is: {} and i am {} years old.", my_name, my_age);
}