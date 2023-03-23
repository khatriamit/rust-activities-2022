// ------------------------------------
//     - Stack
//      - Application of stacks (Expression Evaluation)
// ------------------------------------

fn new_stack(max_size: usize) -> Vec<String> {
    let vec: Vec<String> = Vec::with_capacity(max_size);
    vec
}

fn pop(stack: &mut Vec<String>) -> Option<String> {
    let popped_value = stack.pop();
    popped_value
}

fn push(stack: &mut Vec<String>, item: String, max_size: usize) {
    if stack.len() == max_size {
    } else {
        stack.push(item);
    }
}

fn size(stack: &Vec<String>) -> usize {
    stack.len()
}

/*
RULES for INFIX to POSTFIX evaluation

1. Priorities of operators
    -> +,-
    -> *,/
    -> ^
2. If scanned operator is  <= then the top of the stack in priority,
then pop the operators until we have low priority. Add the popped elements to the
postfix

3. If "(" push it to the stack

4. If ")" pop elements until "(" and add popped elements to postfix

5. If operand then just add to the postfix


RULES for POSTFIX evaluation
    1. If operand -> push to stack
    2. If operation pop two statements perform operation and then push into the stack
*/

fn priority(x: &String) -> u8 {
    if ("+" == x) | ("-" == x) {
        1
    } else if ("*" == x) | ("/" == x) {
        2
    } else if "^" == x {
        3
    } else {
        0
    }
}

fn infix_to_postfix(input: Vec<String>) -> Vec<String> {
    let size_expr = input.len();
    let mut stack = new_stack(size_expr);
    let mut postfix: Vec<String> = Vec::new();

    for i in input {
        match i.as_str() {
            "+" | "-" | "/" | "*" | "^" => {
                if size(&stack) == 0 {
                    push(&mut stack, i, size_expr);
                } else {
                    if priority(&i) > priority(stack.last().unwrap()) {
                        push(&mut stack, i, size_expr)
                    } else {
                        while priority(&i) <= priority(stack.last().unwrap()) {
                            postfix.push(pop(&mut stack).unwrap());
                            if stack.last() == None {
                                break;
                            }
                        }
                        push(&mut stack, i, size_expr)
                    }
                }
            }
            "(" => push(&mut stack, i, size_expr),
            ")" => {
                while stack.last().unwrap() != "(" {
                    postfix.push(pop(&mut stack).unwrap());
                }
                pop(&mut stack).unwrap();
            }
            _ => postfix.push(i),
        }
    }
    while size(&stack) != 0 {
        postfix.push(pop(&mut stack).unwrap());
    }
    println!("{:?}", postfix);
    postfix
}

fn operation(op1: String, op2: String, oper: String) -> f32 {
    let op1 = op1.parse::<f32>().unwrap();
    let op2 = op2.parse::<f32>().unwrap();
    let result = match oper.as_str() {
        "+" => op1 + op2,
        "-" => op1 - op2,
        "*" => op1 * op2,
        "/" => op1 / op2,
        "^" => op1.powf(op2),
        _ => 0.0,
    };
    result
}

fn postfix_evaluation(postfix: Vec<String>) -> f32 {
    let size_expr = postfix.len();
    let mut result_stack: Vec<String> = new_stack(size_expr);
    for i in postfix {
        match i.as_str() {
            "+" | "-" | "/" | "*" | "^" => {
                let oper = i;
                let op2 = pop(&mut result_stack).unwrap();
                let op1 = pop(&mut result_stack).unwrap();
                let result = operation(op1, op2, oper);
                push(&mut result_stack, result.to_string(), size_expr);
            }
            _ => push(&mut result_stack, i.to_string(), size_expr),
        }
    }
    pop(&mut result_stack).unwrap().parse::<f32>().unwrap()
}
fn individual_symbols(input_expr: String) -> Vec<String> {
    let mut tokenized_input: Vec<String> = Vec::new();
    let input_chars: Vec<char> = input_expr.chars().collect();
    let mut temp: Vec<char> = Vec::new();
    for i in input_chars {
        if i != '+' && i != '-' && i != '/' && i != '*' && i != '^' && i != '(' && i != ')' {
            temp.push(i);
            continue;
        } else {
            if temp.len() == 0 {
                tokenized_input.push(i.to_string())
            } else {
                tokenized_input.push(temp.into_iter().collect());
                tokenized_input.push(i.to_string());
                temp = vec![];
            }
        }
    }
    if temp.len() != 0 {
        tokenized_input.push(temp.into_iter().collect());
    }
    println!("Tokenize input: {:?}", tokenized_input);
    tokenized_input
}

fn main() {
    let input_expr = String::from("(33+45/3*(2+9)-50)");
    println!("The original expression is: {:?}", input_expr);
    let input_expr_tokenized = individual_symbols(input_expr);
    let postfix = infix_to_postfix(input_expr_tokenized);
    println!("The evaluated expression = {}", postfix_evaluation(postfix))
}
