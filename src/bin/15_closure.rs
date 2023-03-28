// ------------------------------------------------------
//     - Closure
//          - Rust's anonymous function
//          - Syntax for closure |...| { ... }
//          - Accepting a closure as function arguments
//          - Accepting two closures as function arguments
// ------------------------------------------------------

fn twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {
    f(x) + f(x)
}

fn compose<F, G>(x: i32, f: F, g: G) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    g(f(x))
}
fn main() {
    let add_one = |x: i32| 1 + x;
    println!("The sum of 5 plus 1 is: {} ", { add_one(5) });
    let x: i32 = 10;
    let printer = || {
        println!("the value of x is: {}", x);
    };
    printer();

    let square = |x: i32| x * x;
    let value = twice(5, square);
    println!("The value is: {}", value);

    let multi_closure = compose(5, |n: i32| n + 42, |n: i32| n * 2);
    println!(
        "The value of composite closure function is: {}",
        multi_closure
    );
}
