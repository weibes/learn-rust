use std::io;

fn main() {
    print!("this is a dumb calculator. Please enter your operation: ");
    let mut input = String::new();
    println!();
    io::stdin().read_line(&mut input).unwrap();
    let (a, b, operand) = parse_input(input);
    let result;
    if  operand == '+' {
        result = a + b;
    } else if operand == '-' {
        result = a - b;
    } else if operand == '*' {
        result = a * b;
    } else if operand == '/' {
        if b == 0 {
            panic!("cannot divide by 0...");
        }
        result = a / b;
    } else {
        panic!("{} is not a valid operand", operand);
    }
    println!("first number: { }", a);
    println!("second number: { }", b);
    println!("operand: { }", operand);
    println!("> { }", result);
    println!("thank you come again :)");
}

// assume that the very first character is a number, than space, than an operand, then space,
// than final number. error out if malformed.
fn parse_input(input: String) -> (i32, i32, char) {
    let mut return_tuple: (i32, i32, char) = (0, 0, ' ');
    let mut first_number_done = false;
    let split_strings = input.split_whitespace();
    for curr in split_strings {
        if curr.is_empty() {
            continue;
        }
        if curr.parse::<i32>().is_ok() {
            if !first_number_done {
                return_tuple.0 = curr.parse::<i32>().unwrap(); 
                first_number_done = true;
            } else {
                return_tuple.1 = curr.parse::<i32>().unwrap();
            }
        } else {
            return_tuple.2 =  curr.chars().next().unwrap();
        }
    } 
    return return_tuple;
}
