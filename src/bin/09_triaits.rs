// ------------------------------------
//     - Traits
//          - General Information
//          - Default Implementation
// ------------------------------------

struct Person {
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: i32,
}

struct Student {
    name_std: String,
    age: u8,
    sex: char,
}

struct Circle {
    radius: f32,
}

struct Rectangle {
    length: f32,
    width: f32,
}

trait GeneralInfo {
    fn info(&self) -> (&str, u8, char);
    fn country_info(&self) -> &str;
}

impl GeneralInfo for Person {
    fn info(&self) -> (&str, u8, char) {
        (&self.name, self.age, self.gender)
    }

    fn country_info(&self) -> &str {
        &self.citizenship
    }
}

impl GeneralInfo for Student {
    fn info(&self) -> (&str, u8, char) {
        (&self.name_std, self.age, self.sex)
    }

    fn country_info(&self) -> &str {
        &self.name_std
    }
}

trait ShapeGeneralInfo {
    fn area(&self) {
        println!("I am default implementation of the area")
    }
    fn perimeter(&self);
}

impl ShapeGeneralInfo for Circle {
    fn area(&self) {
        let area_of_circle = 3.14 * (self.radius * self.radius);
        println!("The area of circle is {}", area_of_circle);
    }
    fn perimeter(&self) {
        let circumference = 2.0 * 3.14 * self.radius;
        println!("The perimeter of the circle is {}", circumference);
    }
}

impl ShapeGeneralInfo for Rectangle {
    fn area(&self) {
        let area_of_rectangle = self.width * self.length;
        println!("The area of rectangle is {}", area_of_rectangle);
    }
    fn perimeter(&self) {
        let rectangle_perimeter = 2.0 * (self.width + self.length);
        println!("The perimeter of the rectangle is {}", rectangle_perimeter);
    }
}
fn main() {
    let person1 = Person {
        name: String::from("Bob"),
        age: 30,
        citizenship: String::from("USA"),
        salary: 40_000,
        gender: 'M',
    };

    let student1 = Student {
        name_std: String::from("Alex"),
        age: 10,
        sex: 'M',
    };

    println!("The basic info of person includes {:?}", person1.info());
    println!("The basic info of student includes {:?}", student1.info());

    let c1 = Circle { radius: 5.0 };
    let r1 = Rectangle {
        width: 3.1,
        length: 5.2,
    };

    c1.area();
    c1.perimeter();
    r1.area();
    r1.perimeter();
}
