// ------------------------------------
//     - Break (stopping the loop execution)
//     - Continue (skips the current iteration and next iteration of the loop is executed)
//     -
// ------------------------------------

fn main() {
    // Break
    let mut var = 100;
    loop {
        var = var - 1;
        if var % 13 == 0 {
            break;
        }
    }
    println!(
        "The highest number lesser then the given number divisible by 13 is {}",
        var
    );

    // Continue
    let mut var_1 = 0;
    let mut count = 0;
    let loop_output = loop {
        var_1 = var_1 + 1;
        if var_1 % 5 == 0 && var_1 % 3 == 0 {
            println!("The number divisible by 3 and 5 is {} \n", var_1);
            count = count + 1;
            if count == 3 {
                break var_1;
            } else {
                continue;
            }
        }
    };
    println!("{}", loop_output);
}
