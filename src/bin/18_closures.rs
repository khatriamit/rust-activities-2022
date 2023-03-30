// ------------------------------------------------------
//     - Closures
//          - Basic Syntax
//          - Closure with inputs
//          - Some variable for different closure
//          - Ownership rules and closure
//          - Inference and output (first call to the closure wil set its datatype)
//          - Passing closure as function argument
// ------------------------------------------------------

fn main() {
    let x = 5;
    let square = || println!("The square fo variable x is {} ", x * x);
    square();

    let square_1 = |x| println!("The square of variable x is: {}", x * x);
    let cube = |x: i32| println!("The cube of variable x is: {}", x * x * x);
    square_1(9);

    let y = 10;
    square_1(y);
    cube(3);

    // ---------
    let print_user_age = |general_info: String, name: &str, age: i32| {
        println!("{} \n\t {}:{}", general_info, name, age)
    };

    let general_info = String::from("The details are: ");
    let (person_name, person_age) = (String::from("Alex"), 25);
    print_user_age(general_info, &person_name, person_age);

    // ---------
    let division_status = |y: f32| {
        if y != 0.0 {
            true
        } else {
            false
        }
    };

    fn division<F: Fn(f32) -> bool>(x: f32, y: f32, f: F) {
        if f(y) == true {
            println!("The division result is {}", x / y);
        } else {
            println!("Division is not possible");
        }
    }

    division(5.0, 10.0, division_status);
    division(54.0, 0.0, division_status);
}
