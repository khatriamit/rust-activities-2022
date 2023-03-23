// ------------------------------------
//     - Stack
//      - Application of stacks (reverse string)
// ------------------------------------

fn new_stack(max_size: usize) -> Vec<char> {
    let vec: Vec<char> = Vec::with_capacity(max_size);
    vec
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    let popped_value = stack.pop();
    popped_value
}

fn push(stack: &mut Vec<char>, item: char, max_size: usize) {
    if stack.len() == max_size {
    } else {
        stack.push(item);
    }
}

fn size(stack: &Vec<char>) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");
    let n: u32 = n.trim().parse().expect("Invalid input");
    n
}

fn main() {
    let input_string = String::from("Welcome to the Rust");
    let size_stack = input_string.len();

    let mut stack = new_stack(size_stack);
    let mut rev_string = String::new();
    for i in input_string.chars() {
        push(&mut stack, i, size_stack);
    }

    for i in 0..size(&stack) {
        rev_string.push(pop(&mut stack).unwrap());
    }

    println!("the input string is {:?}", input_string);
    println!("the reverse of string is {:?}", rev_string);
}
