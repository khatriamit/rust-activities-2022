// ------------------------------------
//     - Match
//          - simple match statement
//          - if let syntax style match
// ------------------------------------

/*
General Syntax
--------- simple match statement -----------
match value {
    possible_values(s) => {statements to execute},
    possible_values(s) => {statements to execute},
    possible_values(s) => {statements to execute},
    _=> {default execution statement}
}
--------- simple match statement -----------

--------- if let syntax style match -----------
let variable =match value {
    possible_values(s) => {statements to execute},
    possible_values(s) => {statements to execute},
    possible_values(s) => {statements to execute},
    _=> {default execution statement}
};
--------- if let syntax style match -----------

*/

fn main() {
    // simple match statement
    let some_number = 100;
    match some_number {
        1 => println!("The number is 1"),
        2 | 3 => println!("The number is 2 or 3"),
        4..=100 => println!("The number is between 4 and 100 inclusive"),
        _ => println!("The number is greater than 100"),
    }

    let marks = 50;
    let mut grade = 'N';

    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        60..=69 => grade = 'D',
        _ => grade = 'F',
    }
    println!("The grade achieved is {} ", grade);

    // if let syntax style match
    let grade_1 = match marks {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };
    println!("The grade_1 achieved is {} ", grade_1);
}
