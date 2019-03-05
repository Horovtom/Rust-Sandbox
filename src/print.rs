pub fn run() {
    // Print to console
    println!("I can print to console!");

    // Lets print a simple number:
    println!("The number is: {}", 1);
    // We can see that this is behaving much like python format string statements...

    // Now we want to print this statement:
    // Brad is from Mass and Brad likes to code
    println!("{0} is from {1} and {0} likes to {2}.", "Brad", "Mass", "code");

    // We can do similar thing but in much less cryptic fashion:
    println!("{name} is from {place} and {name} likes to {activity}.", name = "Brad", place = "Mass", activity = "code");

    // We can use traits for formatting:
    println!("Binary: {:b}, hexadecimal: {:x}, octal: {:o}", 10, 10, 10);
    // No idea if this will work:
    println!("Binary: {number:b}, hexadecimal: {number:X}, octal: {number:o}", number = 10);

    // We can write debug info with this trait:
    println!("{:?}", (12, true, "Hello"));
}