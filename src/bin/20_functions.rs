// ------------------------------------------------------
//     - Function Types
//          - Basic syntax and use
//          - Function type as parameter to another function
//          - Functional pointers
//          -
// ------------------------------------------------------

fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn min(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}

fn prints_name(name: &str) {
    println!("The name is: {}", name);
}

fn prints_full_info(f: fn(&str), some_name: &str, age: i32) {
    f(some_name);
    println!(" and my age is {}", age);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
fn main() {
    // let max_result = max(1, 2);
    let mut f = min;
    println!("The minimum of two value is: {}", f(2, 3));

    let mut f = max;
    println!("The max of two value is: {}", f(2, 3));

    // ------------
    let (my_name, my_age) = (String::from("Alex"), 40);
    prints_full_info(prints_name, &my_name, my_age);

    // ------------
    let response = do_twice(add_one, 5);
    println!("The response is: {}", response);
}
