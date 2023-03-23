// ------------------------------------
//     - Stack
//      - stack using vec
// ------------------------------------

fn new_stack(max_size: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(max_size);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let popped_value = stack.pop();
    println!("The popped value is {:?}", popped_value);
    popped_value
}

fn push(stack: &mut Vec<u32>, item: u32, max_size: usize) {
    if stack.len() == max_size {
        println!("Cannot add more elements")
    } else {
        stack.push(item);
        println!("Stack: {:?}", stack);
    }
}

fn size(stack: &Vec<u32>) -> usize {
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
    println!("Let us create stack");
    println!("Please define the size of the stack");

    let size_stack = input();
    let mut stack = new_stack(size_stack as usize);

    loop {
        println!("\n\n ****** menu ******* \n");
        println!(" 1. Push\n 2. Pop\n 3. Display\n 4. Size\n 5. Exit");

        let choice = input();
        match choice {
            1 => {
                println!("Enter the value to be inserted");
                let item = input();
                push(&mut stack, item, size_stack as usize);
            }
            2 => println!("The element which is popped is {:?}", pop(&mut stack)),
            3 => println!("The elements in stack are: {:?}", stack),
            4 => println!("The size of the stack is {}", size(&stack)),
            5 => break,
            _ => println!("\n Wrong selection !!! Try again !!!"),
        }
        println!("Do you want to continue 1 = Yes / 0 = No");
        let status = input();
        if status == 1 {
            continue;
        } else {
            break;
        }
    }
}
