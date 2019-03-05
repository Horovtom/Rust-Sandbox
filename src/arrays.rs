// Fixed lists with the same datatype

pub fn run() {
    let numbers: [i32;5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single val:
    println!("Signle value: {}", numbers[0]);

    let mut numbers2: [i32;5] = [1,2, 3,4 5];
    // Re-assign value
    numbers2[0] = 20;

    // Get array length
    println!("Length of array: {}", numbers2.len());

    // Get the allocated stack size
    println!("This array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers2[0..2];
    println!("Slice: {:?}", slice);
}