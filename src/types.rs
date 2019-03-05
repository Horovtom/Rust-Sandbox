/*
Rust is a statically typed language, which means that the compiler has to know the types
of all the variables at compile time. However, the compiler can usually infer a lot of the 
types based on the assigned value.

Primitive types - 
    Integers: u8, i8, u16, i16, ... u128, i128
    Floats: f32, f64
    Boolean: (bool)
    Characters: (char)
    Tuples
    Arrays
*/

pub fn run() {
    // Default is i32
    let a = 12;

    // Default is f64
    let b = 0.12;

    // Add explicit type:
    let c: i8 = 16;

    // Find max size:
    println!("Max i64 is: {}", std::i64::MAX);

    // Boolean 
    let isActive = true;
    println!("{:?}", (a, b, c, isActive));

    // Get a boolean from an expression:
    let is_greater = 10 > 20;

    // Characters:
    let a1 = 'a';
    // Characters are unicode, so we can do this:
    let a2 = '\u{1F600};
    println!("{:?}", (a1, a2));
    

}