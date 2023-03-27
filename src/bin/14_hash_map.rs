// ------------------------------------------------------
//     - Hash Maps
//          - Defining Hashmaps
//          - Inserting values in hashmap
//          - Accessing the value from HashMap
//          - Check if the value exists
//          - Iterating through elements of hashmap
//          - Updating the value at specific key
// ------------------------------------------------------

use std::collections::HashMap;

fn main() {
    let mut person: HashMap<&str, i32> = HashMap::new();
    person.insert("Alex", 40);
    person.insert("Bob", 51);
    person.insert("John", 41);

    println!(
        "The age of person is = {:?}",
        person.get("Alex").expect("The value not found")
    );
    if person.contains_key("Alex") {
        println!("The person exists");
    } else {
        println!("The value doesn't exists");
    }

    match person.get("Alex") {
        Some(value) => println!("The value exists {}", value),
        None => println!("The value does not exits"),
    }

    for (name, age) in &person {
        println!("The name of person is {} and his/her age is {}", name, age);
    }

    let mut likes: HashMap<&str, &str> = HashMap::new();
    likes.insert("Alex", "apple");
    likes.insert("Alex", "orange");
    println!("{:?}", likes);

    likes.entry("Alex").or_insert("apple");
    println!("with entry method{:?}", likes);

    let some_vec: Vec<i32> = vec![5, 5, 8, 8, 1, 0, 1, 5, 5, 5, 5];
    let mut feq_vec: HashMap<i32, u32> = HashMap::new();
    for i in &some_vec {
        let freq: &mut u32 = feq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }
    println!("{:?}", feq_vec);
}
