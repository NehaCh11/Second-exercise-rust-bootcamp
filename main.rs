enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    use Operation::*;

    match operation {
        Add(a, b) => a + b,
        Subtract(a, b) => a - b,
        Multiply(a, b) => a * b,
        Divide(a, b) => {
            if b != 0.0 {
                a / b
            } else {
                println!("Cannot divide by zero.");
                0.0
            }
        },
    }
}

fn main() {
    use std::io;

    println!("Enter the first number:");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number).expect("Failed to read line");
    let first_number: f64 = first_number.trim().parse().expect("Please type a number!");

    println!("Enter the operation (Add, Subtract, Multiply, Divide):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");

    println!("Enter the second number:");
    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number).expect("Failed to read line");
    let second_number: f64 = second_number.trim().parse().expect("Please type a number!");

    let operation = operation.trim();
    let operation_enum = match operation.to_lowercase().as_str() {
        "add" => Operation::Add(first_number, second_number),
        "subtract" => Operation::Subtract(first_number, second_number),
        "multiply" => Operation::Multiply(first_number, second_number),
        "divide" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation.");
            return;
        }
    };

    let result = calculate(operation_enum);
    println!("Result: {}", result);
}

