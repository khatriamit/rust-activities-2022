// ------------------------------------------------------
//     - Lifetimes
//          - Generic Lifetime Parameters
//          - GLP are typically needed with outputs from functions that are references
//          - Issue with GLP
//          - GLP with multiple variables
//          - GLP and structures
//          - Reference to same variable
// ------------------------------------------------------

fn some_fn<'a, 'b>(first_str: &'a str, second_str: &'b str) -> &'a str {
    first_str
}

fn greater<'a>(i: &'a i32, j: i32) -> &'a i32 {
    i
}

fn greater1<'a, 'b>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if i > j {
        i
    } else {
        j
    }
}

struct Person<'a> {
    name: &'a str,
    age: i32,
}

fn main() {
    let s_1 = "Hello";
    let v;
    {
        let s_2 = String::from("World");
        v = some_fn(s_1, s_2.as_str())
    }
    println!("{}", v);

    let int1 = 5;
    let int2 = 10;
    // let result = greater(&int1, int2);
    let result1 = greater1(&int1, &int2);

    let first_name = "John";
    let mut john = Person {
        name: &first_name,
        age: 40,
    };
    {
        let last_name = String::from("Doe");
        john.name = &last_name;
    }
    // println!(
    //     "The name of the person is {} and his age is {}",
    //     john.name, john.age
    // );
}
