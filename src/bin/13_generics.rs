// ------------------------------------------------------
//     - Generics
//          - Motivation (reducing code duplication)
//          - Generics in function
//          -
// ------------------------------------------------------

fn squarei32(x: i32) -> i32 {
    x * x
}

fn squaref32(x: f32) -> f32 {
    x * x
}

fn square<T: std::ops::Mul<Output = T> + Copy>(x: T) -> T {
    x * x
}
fn main() {
    println!("The square of 5 is  {}", squarei32(5));
    println!("The square of 5.5 is  {}", squaref32(5.5));
    println!("The square of 5 using generic function is {}", square(5));
    println!(
        "The square of using generic function  5.5 is {}",
        square(5.5)
    );
}
