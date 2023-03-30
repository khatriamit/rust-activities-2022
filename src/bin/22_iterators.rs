// ------------------------------------------------------
//     - Iterators
//          - Modifying and collecting values
//          -
// ------------------------------------------------------

fn main() {
    let a = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let response = a.iter().filter(|&x| *x >= 5).collect::<Vec<&u32>>();
    println!("The filtered values {:?}", response);

    let b = a.clone();
    let response = b
        .into_iter()
        .filter(|x| *x >= 5)
        .map(|x| x * 2)
        .collect::<Vec<u32>>();
    println!("The filtered values {:?}", response);
}
