// Primitive str - immutable fixed-length string somewhere in memory
// String - growable heap-allocated.

pub fn run() {
    // The immutable version:
    let hello = "hello";

    // The heap-allocated:
    let hello2 =  String::from("hello2");

    // Get length
    println!("length of {} is {} and length of {} is {}.", 
    hello, hello.len(), hello2, hello2.len());

    // Let's apped:
    let mut a = String::from("Hello ");
    a.push('W');
    a.push_str("orld");

    // Capacity in bytes
    println!("Capacity is: {}", a.capacity());

    // Contains
    println!("Contains 'World': {}", a.contains("World"));

    // Replace
    println!("Replace: {}", a.replace("World", "There"));

    // Loop through whitespace
    for word in a.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(3, s.len());

}