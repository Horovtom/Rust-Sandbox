// Vectors are resizable arrays

pub fn run() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single val:
    println!("Signle value: {}", numbers[0]);

    let mut numbers2: [i32;5] = [1,2, 3,4 5];
    // Re-assign value
    numbers2[0] = 20;

    // Add on to vector:
    numbers2.push(6);

    // Pop:
    numbers2.pop();


    // Get vector length
    println!("Length of vector: {}", numbers2.len());

    // Get the allocated stack size
    println!("This vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers2[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
        for x in numbers2.iter() {
            println!("{}", x);  
        }

    // Loop and mutate valeus:
    for x in numbers2.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers2);
    
}