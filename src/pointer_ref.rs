// Reference pointers point to position in memory

pub fn run() {
    // Primitive array
    let array1 = [1, 2,3 ,4];
    let array2 = array1;

    println!("Values: {:?}" , (array1, array2));

    // With non-primitives, if you assign another variable 
    // to a piece of data, the first variable will no longer 
    // hold that value. You'll need to use a reference (&) 
    // to point  to the resource.

    let vec1 = vec![1,2,3,4];
    let vec2 = &vec1; 
    println!("Values: {:?}", (&vec1, vec2));
}