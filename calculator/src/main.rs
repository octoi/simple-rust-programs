use std::env::{Args, args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).expect("Failed to get first number");
    let operator = args.nth(0).expect("Failed to get operator").chars().next().expect("Failed to get operator");
    let second = args.nth(0).expect("Failed to get second number");

    let first_number = first.parse::<f32>().expect("Invalid number");
    let second_number = second.parse::<f32>().expect("Invalid number");

    let result = operate(operator, first_number, second_number);

    println!("{}", output(first_number, operator, second_number, result));
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' => first_number * second_number,
        _ => panic!("Invalid operator used!")
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}