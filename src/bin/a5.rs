/*OWNERSHIP and BORROWING */
fn main() {
    /*
    here the value of x is copied to the new location i.e y
    it is possible if the memory allocation is in the stack but
    it won't work if the memory is allocated in the heap i.e using box
    */

    let x = 45;
    let y = x;

    // x += 10;
    println!("x = {x}");
    println!("y = {y}");

    /*
        data is moved form u to v so it doesn't works
    */
    let u = Box::new(30);
    let v = u;

    // println!("u = {u}");
    println!("v = {v}");

    /*
        unlike other data types string is allocated inside heap
    */
    let name = String::from("Bob");
    let mut temp_name = name; //name is moved to temp_name
    temp_name.push_str(" Marley");

    println!("Name is {temp_name}");
    // println!("Name is {name}");

    struct Employee {
        name: String,
        role: String,
    }

    let emp1 = Employee {
        name: "Jonh".to_owned(),
        role: String::from("HR"),
    };

    let Employee { name, ref role } = emp1;
    println!("{}", name);
    println!("{}", role);
    // println!("emp name = {}", emp1.name)
    println!("emp role = {}", emp1.role);

    let name2 = String::from("Jack");
    hello(&name2);

    let mut name3 = String::from("Hello");
    update_hello(&mut name3);
}

/*
    here hello takes one argument name of type string but we have used
    & before data type which means it is the reference to the heap allocated
    memory. So on using it we are not passing data to the function but instead
    we are passing the memory location of the name and the memory ownership
    is still with the main function not with the hello.
*/
fn hello(name: &String) {
    println!("Hello {name}");
}

/*
    if heap data need to be updated we should pass argument as mutable reference
    and also the variable is to be declared as mutable.
*/
fn update_hello(name: &mut String) {
    name.push_str(" World");
    println!("updated {name}");
}
