// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    //let mut a = Vec::with_capacity(100); // capacity ≠ len
    //capacity = 100, len = 0

 /*   assert_eq!(a.len(), 0);
    assert_eq!(a.capacity(), 100);

    for i in 0..a.capacity() {
        a.push(i+1);
    }

    assert_eq!(a.len(), 100);
    assert_eq!(a.capacity(), 100);*/

    // All elements can be initialized to the same value
    let a: [i32; 100] = [0; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
/*    println!("initial capacity: {}", a.capacity());

    a.push(101); // double the initial capacity
    a.push(102);
    println!("a.len after extension: {}", a.len());

    assert_eq!(a.len(), 102);
    println!("a.capacity after extension: {}", a.capacity());*/
    //assert_eq!(a.capacity(), 100);
}
