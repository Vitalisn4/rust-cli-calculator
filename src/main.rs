use std::io::{self, Write};
use std::fs::OpenOptions;
use meval::eval_str;  // Importing meval crate for expression evaluation

// Function to perform basic arithmetic operations
fn calculate(num1: f64, operator: &str, num2: Option<f64>) -> Result<f64, String> {
    match operator {
        "+" => Ok(num1 + num2.unwrap()),
        "-" => Ok(num1 - num2.unwrap()),
        "*" => Ok(num1 * num2.unwrap()),
        "/" => {
            if num2.unwrap() == 0.0 {
                Err("Error: Division by zero".to_string())
            } else {
                Ok(num1 / num2.unwrap())
            }
        }
        "%" => Ok(num1 % num2.unwrap()),
        "^" => Ok(num1.powf(num2.unwrap())),
        "fact" => {
            if num1 < 0.0 {
                Err("Error: Factorial of a negative number is undefined".to_string())
            } else {
                Ok((1..=num1 as u64).product::<u64>() as f64)
            }
        }
        _ => Err("Error: Invalid operation".to_string()),
    }
}

// Function to process and evaluate expressions using meval crate
fn evaluate_expression(expression: &str) -> Result<f64, String> {
    match eval_str(expression) {
        Ok(result) => Ok(result),
        Err(_) => Err("Error: Invalid expression format".to_string()),
    }
}

// Function to log history
fn log_history(entry: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("history.txt")
        .expect("Unable to open history file");
    writeln!(file, "{}", entry).expect("Unable to write to file");
}

// Function to read user input
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Flush output buffer for cleaner display
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

// Main function
fn main() {
    loop {
        let mode = get_input("Do you want to do an inline calculation or evaluate an expression? (inline/express): ");

        match mode.to_lowercase().as_str() {
            "inline" => {
                // Inline calculation mode
                let num1: f64 = match get_input("Enter first number: ").parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid number.");
                        continue;
                    }
                };

                let operator = get_input("Enter operation (+, -, *, /, %, ^, fact): ");

                let num2 = if operator != "fact" {
                    match get_input("Enter second number: ").parse() {
                        Ok(n) => Some(n),
                        Err(_) => {
                            println!("Invalid input. Please enter a valid number.");
                            continue;
                        }
                    }
                } else {
                    None
                };

                match calculate(num1, &operator, num2) {
                    Ok(result) => {
                        let history_entry = format!("{} {} {} = {}", num1, operator, num2.unwrap_or(0.0), result);
                        println!("Result: {}", result);
                        log_history(&history_entry);
                    }
                    Err(e) => println!("{}", e),
                }
            }
            "express" => {
                // Expression evaluation mode
                let expression = get_input("Enter an expression (e.g., 10 + 2 * 3): ");
                match evaluate_expression(&expression) {
                    Ok(result) => {
                        println!("Result: {}", result);
                        log_history(&expression);
                    }
                    Err(e) => println!("{}", e),
                }
            }
            _ => {
                println!("Invalid option. Please choose 'inline' or 'express'.");
                continue;
            }
        }

        if get_input("Do you want to perform another calculation? (yes/no): ").to_lowercase() != "yes" {
            break;
        }
    }
}
