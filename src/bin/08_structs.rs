// ------------------------------------
//     - Structures
//      - Defining the structure
//      - Tuple Structs
// ------------------------------------

struct Person {
    name: String,
    age: u32,
    gender: char,
    citizenship: String,
    salary: i32,
}

impl Person {
    fn new() -> Self {
        Self {
            name: String::from("Bob"),
            age: 30,
            gender: 'M',
            citizenship: String::from("USA"),
            salary: 15_000,
        }
    }
    fn compute_taxes(&self) -> f32 {
        (self.salary as f32 / 3.) * 0.5
    }
}
fn main() {
    let person1 = Person {
        name: String::from("Alex"),
        age: 30,
        gender: 'M',
        citizenship: String::from("England"),
        salary: 45_000,
    };
    println!(
        "The structure values are {}, {}, {}",
        person1.name, person1.age, person1.gender
    );

    println!(
        "The computed taxes on person {} is {}",
        person1.name,
        person1.compute_taxes()
    );

    let person2 = Person::new();
    println!(
        "The person2 structure values are {}, {}, {}",
        person2.name, person2.age, person2.gender
    );

    let person3 = Person {
        age: 50,
        name: String::from("Jack"),
        ..person1
    };
    println!(
        "The person3 structure values are {}, {}, {}",
        person3.name, person3.citizenship, person3.salary
    );
}
