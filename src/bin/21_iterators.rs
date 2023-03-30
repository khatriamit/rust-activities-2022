// ------------------------------------------------------
//     - Iterators
//          - Basics of iterators and its syntax
//          - Some useful functions for iterators
// ------------------------------------------------------

fn main() {
    let some_vec = vec![1, 2, 3, 4, 5, 6, 7];
    let mut iter = some_vec.iter();

    println!("The values of iterator: {:?}", iter);
    println!("The values next of iterator: {:?}", iter.next());
    println!("The values next of iterator: {:?}", iter.next());
    println!("The values next of iterator: {:?}", iter.next());
    println!("The values next of iterator: {:?}", iter.next());
    println!("The values next of iterator: {:?}", iter.next());
    println!("The values next of iterator: {:?}", iter.next());
    println!("The values next of iterator: {:?}", iter.next());
    println!("The values next of iterator: {:?}", iter.next());
    println!("The values next of iterator: {:?}", iter.next());

    //------------------
    let a: Vec<u32> = vec![0, 2, 1, 3, 4, 5];
    let check = a.iter().any(|&x| x > 5);
    println!("The value of {}", check);

    let check = a.iter().all(|&x| x > 4);
    println!("The value of {}", check);

    let find = a.iter().find(|&&x| x > 2);
    println!("The response of find is: {}", find.unwrap());

    let find_posn = a.iter().position(|&x| x == 4);
    println!("The position of given value is: {}", find_posn.unwrap());

    let find_extreme_r_posn = a.iter().rposition(|&x| x == 4);
    println!(
        "The position of given value is: {}",
        find_extreme_r_posn.unwrap()
    );

    let check_max = a.iter().max();
    println!("The max value in vec is: {}", check_max.unwrap());

    let check_min = a.iter().min();
    println!("The min value in vec is: {}", check_min.unwrap());

    let mut iter = a.iter().rev();
    println!("The rev is: {:?}", iter);
    println!("The 1st vale form rev is: {:?}", iter.next());
}
