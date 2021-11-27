use std::mem;

fn analyze_slice(arr: &[i32]) {
    println!("first element of the slice: {}", arr[0]);
    println!("the slice has {} elements.", arr.len());
}

fn main() {
    //fixed sized array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    
    //all elements can be intialized to the same value
    let ys: [i32; 500] = [1; 500];

    println!("4th element of the first array is: {}", xs[3]);
    println!("41st element of the second array: {}", ys[42]);
    println!("size of the first and second array: {} {}", xs.len(), ys.len());

    // arrays are stack allocted
    println!("array copies {} bytes", mem::size_of_val(&xs));

    // we can borrow the whole array as slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // we can borrow the section of an array as slice
    analyze_slice(&ys[0 .. 500]);
}