#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn obj(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }

    fn add(&mut self, a: i32, b: i32) {
        self.x += a;
        self.y += b;
    }
}

fn main() {
    let result = add(5, 6);
    dbg!(result);

    let mut p1 = Point { x: 1, y: 3 };
    dbg!(&p1);

    let p2 = Point::new(4, 5);
    dbg!(&p2);

    p1.obj();
    p2.obj();
    p1.add(4, 5);
    p1.obj();
    Point::obj(&p1);
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
