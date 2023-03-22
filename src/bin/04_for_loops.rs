// ------------------------------------
//     - For loops
//      - Simple for loop
//      -
// ------------------------------------

fn main() {
    let mut some_vec = vec![10, 11, 12, 13, 14, 15];
    for i in 0..5 {
        println!("The {}th value in the vector is {}", i, some_vec[i]);
    }

    for i in some_vec.iter() {
        println!("The  value in the vector is {}", i);
    }

    for i in some_vec.iter_mut() {
        *i += 5;
        println!("The updated value in the vector is {}", i);
    }
}
