// ------------------------------------
//     - Loops
//          - loop with no condition
//          - while loop (repeats the execution of code unknown time until the condition is matched)
// ------------------------------------

fn main() {
    loop {
        println!("This is an infinite loop");
        break; // let's break it for now
    }
    let my_number = 5;
    let mut guess = false;

    println!("Guess the number between 1 and 20");
    while guess != true {
        let mut number = String::new();
        std::io::stdin()
            .read_line(&mut number)
            .expect("failed to read input");
        let number = number.trim().parse().expect("Invalid input");
        if my_number == number {
            println!("You guessed the number correct.");
            guess = true;
        } else {
            println!("Try again please")
        }
    }

    let mut number_1 = String::new();
    std::io::stdin()
        .read_line(&mut number_1)
        .expect("failed to read input");
    let mut number_1: u8 = number_1.trim().parse().expect("Invalid input");
    number_1 += 1;
    while (number_1 % 2 == 0 && number_1 % 5 == 0) != true {
        number_1 = number_1 + 1;
    }
    println!(
        "The number after your number which is divisible by both 2 and 5 is {}",
        number_1
    );
}
