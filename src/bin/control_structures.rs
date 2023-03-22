// ------------------------------------
//     - Conditional If
//     - If with multiple conditions
//     - If else
// ------------------------------------

/*
General Syntax

if condition {
    // statement to execute if condition proves true
}
else {
    // statement to execute if condition proves false
}
 ************* multiple condition **************
if condition 1 {
    // statement to execute if condition1 proves true
}
else if condition 2 {
    // statement to execute if condition2 proves true
}
else if condition 3 {
    // statement to execute if condition3 proves true
}
else {
    // statement to execute if none condition proves true
}
 ************* multiple condition **************

*/

fn main() {
    let some_number = 40;
    if some_number < 50 {
        println!("****if block is executed*****")
    }

    let flag_1 = true;
    let flag_2 = true;

    if flag_1 == true || flag_2 == false {
        println!("One of the statement is TRUE");
    }
    if flag_1 == false || flag_2 == true {
        println!("One of the statement is FALSE");
    }
    if flag_1 == true && flag_2 == true {
        println!("Both of the statement are TRUE")
    } else {
        println!("Not TRUE")
    }

    if flag_1 != false {
        println!("Not FALSE")
    }

    /* Complex condition */
    if (flag_1 == true && flag_2 == false) || some_number > 50 {
        println!("The complex condition is TRUE");
    } else {
        println!("The complex condition is FALSE");
    }
}
