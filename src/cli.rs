use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    println!("Args: {:?}", args);
    let command = args[1].clone();
    println!("Command: {}", command);

    if command == "hello" {
        println!("Hi, Brad");
    } else {
        println!("Go away");
    }
}