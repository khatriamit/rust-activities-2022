// ------------------------------------------------------
//     - Closures
//          - Last recap
//          - Borrow by immutable reference
//          - Borrow by mutable reference
//          - Moving of a value inside the closure
// ------------------------------------------------------

fn main() {
    let some_closure = |x: u32| -> u32 { x + 1 };
    let some_closure_2 = |x| x + 1;

    some_closure(2);
    some_closure_2(2);

    // -----------
    let mut vec_1 = vec![1, 2, 3];
    let some_closure_3 = || println!("Vec 1: {:?}", vec_1);
    println!("{:?}", vec_1);
    some_closure_3();

    // -----------
    let mut some_closure_4 = || vec_1.push(4);
    some_closure_4();
    vec_1[0] = 0;
    println!("{:?}", vec_1);

    // -----------
    let mut some_closure_5 = || {
        let vec_2 = vec_1;
    };
    some_closure_5();
    // println!("vec 1= {:?}", vec_1);
    // println!("vec 2= {:?}", vec_2);
}
