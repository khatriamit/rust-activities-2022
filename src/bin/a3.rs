/*GENERICS */

fn main() {
    echo_back(5);
    echo_back(5.5);
    let _msg = echo_back("hello");
    print(5);
    print(5.5);
    print("hello");
    dbg!(add(5, 6));
    dbg!(add(5.5, 9.5));
    debug_print("hello Bob");

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.9, y: 2.5 };

    dbg!(p1);
    dbg!(p2);

    let p3 = Point::new(1.6, 5.7);
    dbg!(p3);
}

fn echo_back<T>(x: T) -> T {
    x
}

fn print<T: std::fmt::Display>(x: T) {
    println!("{x}");
}

fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn debug_print<T>(x: T)
where
    T: std::fmt::Display + std::fmt::Debug,
{
    println!("{:#?}", x);
}
