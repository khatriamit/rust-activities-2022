// -------------------------------------------------------------------------
//     - Rust Module
//          - Binary crates: can execute
//          - Library crate: can be used as module for other crate (binary)
//              - Functions within the module can access each other without the pub keyword
//              - Functions outside the module cannot be accessed without the pub keyword
//              - crate::file_name::fn_name can be used to access the fn within the child module
// -------------------------------------------------------------------------

mod maths {
    pub mod basic_math {
        pub fn multiplication(num1: &i32, num2: &i32) -> i32 {
            let result = num1 * num2;
            printing(&result);
            result
        }

        fn printing(num: &i32) {
            println!("The result is {}", num)
        }
    }
}

fn rect_area(length: &i32, width: &i32) -> i32 {
    maths::basic_math::multiplication(length, width)
}

struct Rectangle {
    length: i32,
    width: i32,
}
fn main() {
    let rect1 = Rectangle {
        length: 10,
        width: 5,
    };

    let area = rect_area(&rect1.length, &rect1.width);
    println!("The area is {}", area);
}
