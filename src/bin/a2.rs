/*CLOSURES */

fn main() {
    let add = |x: i32, y: i32| -> i32 { x + y };
    let sub = |x: i32, y: i32| -> i32 { x - y };

    dbg!(add(5, 5));
    dbg!(sub(8, 3));

    let name = String::from("Amit");
    /*
        this closure is also called environment capturer
        it can capture the variables from outside it's scope too
        it is immutable reference as in line 18 the name variable
            can still be accessed
    */
    let hi = || println!("Hi! {name}");
    hi();
    dbg!(&name);

    /*
        if you require mutable closure we need to define
            the variable as mut and also the closure name as
            mut
    */

    let mut name2 = String::from("Amit");
    let mut hi2 = || {
        name2.push_str(" Khatri");
        println!("Hi!, {name2}");
    };
    hi2();

    /*
        If you want to pass the ownership of the variable
        in the closure, you need to use move keyword.
        In line number 40, the memory allocated for name
        inside the heap is deallocated and cannot be
        used further so we get error in line 43
    */

    let hi3 = move || println!("Hi! {name}");
    hi3();
    // println!("Hi! {name}")

    /*
        Passing function as na argument in another
        function.
    */
    let hi3 = || println!("Hi");
    fn hi_there() {
        println!("Hi, there");
    }
    fn call_me(f: impl Fn()) {
        f();
    }
    call_me(hi3);
    call_me(hi_there);
}
