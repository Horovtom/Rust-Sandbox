pub fn run() {
    let mut count = 0;
    
    // Infinite loop
    loop {
        count += 1;
        println!("Number is {}", count);

        if count == 20 {
            break;
        }
    }

    count = 0;

    // While Loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if (count % 3 == 0) {
            println!("Fizz");
        } else if (count % 5 == 0) {
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        count+=1;
    }

    // For range loop 
    for x in 0..100 {
        if x % 2 == 0 {
            println!("Haha");
        } else {
            println!("Hihi");
        }
    }
}