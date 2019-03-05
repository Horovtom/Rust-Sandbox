pub fn run() {
greeting("Hello", "Jane");
    // BInd function values to variables
    let get_sum = add(2, 3);
    println!("{}", get_sum);

    // Closure
    let n3 = 16;
    let add_nums = |n1:i32, n2:i32| n1 + n2 + n3;
    println!("C sum {} ", add_nums(6, 4));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(a:i32, b:i32) -> i32 {
    // We return like this
    a + b
}