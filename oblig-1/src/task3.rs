// Task 3: Create a mini calculator

use std::io::{stdin, stdout, Write};

enum Operation {
    Add,
    Subtract,
    Multiply,
    Devide,
}
impl Operation {
    fn calculate(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
            Self::Multiply => x * y,
            Self::Devide => x / y,
        }
    }
}

fn get_user_input(question: &str) -> String {
    //Create mutatable string to bind user inputs too
    let mut return_value = String::new();

    //Ask Question
    print!("{} :", question);

    //Clear user input
    let _ = stdout().flush();

    //Read user input
    stdin()
        .read_line(&mut return_value)
        .expect("failed to provide string!");

    //Remove newline characters
    if let Some('\n') = return_value.chars().next_back() {
        return_value.pop();
    }

    //Remove return character
    if let Some('\r') = return_value.chars().next_back() {
        return_value.pop();
    }

    return_value
}

pub fn run() {
    let user_num1: i32 = get_user_input("First number for calculator")
        .parse::<i32>()
        .unwrap_or(0);
    let user_operation = get_user_input("Operator for calculator");
    let user_num2: i32 = get_user_input("Last number for calculator")
        .parse::<i32>()
        .unwrap_or(0);

    //Convert string to operator
    let operation = match user_operation.as_str() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Devide,
        _ => Operation::Add,
    };

    //Run Calculation
    let result = operation.calculate(user_num1, user_num2);

    println!(
        "{} {} {} = {}",
        user_num1, user_operation, user_num2, result
    )
}
