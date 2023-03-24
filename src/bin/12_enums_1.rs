// ------------------------------------------------------
//     - Enums
//          - General Syntax
//          - Eum with attached data
//          - Enums to crete vectors with different data types
// ------------------------------------------------------

#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f32),
}
fn main() {
    let some_value = vec![Value::Integer(12), Value::Float(15.5)];
    println!(
        "The value of the integer is {:?} and the value of float is  {:?}",
        some_value[0], some_value[1]
    );
    for i in some_value.iter() {
        match i {
            Value::Integer(num) => println!("The value is an integer with the value of {}", num),
            Value::Float(num) => println!("The value is an float with the value of {}", num),
        }
    }
}
