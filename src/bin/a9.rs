/*STRUCT AND LIFETIMES*/

/* Struct workshop */
struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

struct Coordinates(i32, i32, i32);
struct UnitStruct;

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn calc_perimeter(&self) -> u32 {
        self.height + self.width
    }
    fn change_values(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
    }
}

fn build_user(username: String) -> User {
    User {
        active: true,
        username,
        sign_in_count: 10,
    }
}

fn main() {
    /* Struct workshop */
    let user1 = User {
        active: true,
        username: String::from("Amit"),
        sign_in_count: 1,
    };
    println!("Username: {}", user1.username);
    println!("Is active: {}", user1.active);
    println!("Total sign in count: {}", user1.sign_in_count);
    let user2 = build_user(String::from("amit"));
    println!("user 2 {}", user2.username);

    let _coords = Coordinates(1, 2, 3);

    let mut sq = Square {
        width: 5,
        height: 5,
    };
    println!("The area of sq is {}", sq.area());
    println!("The perimeter of sq is  {}", sq.calc_perimeter());
    sq.change_values(10, 10);
    println!("The perimeter of new sq is  {}", sq.calc_perimeter());

    /* Struct Lifetime workshop */
    let str1 = String::from("This is demo string");
    let x = MyString {
        text: str1.as_str(),
    };
    println!("The string value is: {}", x.text);
}
/* Lifetime helps to prevent dangling reference */
// &i32
// &'a i32
// &'a mut i32

fn _example<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

struct MyString<'a> {
    text: &'a str,
}
