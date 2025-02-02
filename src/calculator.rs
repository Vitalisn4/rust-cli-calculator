use std::fs::OpenOptions;
use std::io::{self, Write};
use meval::eval_str;

// Define a Calculator struct to hold methods and logic
pub struct Calculator;

impl Calculator {
    // Function to perform basic arithmetic operations, including square root
    pub fn calculate(num1: f64, operator: &str, num2: Option<f64>) -> Result<f64, String> {
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
            "sqrt" => {
                if num1 < 0.0 {
                    Err("Error: Square root of a negative number is undefined".to_string())
                } else {
                    Ok(num1.sqrt())
                }
            }
            _ => Err("Error: Invalid operation".to_string()),
        }
    }

    // Function to process and evaluate expressions using meval crate
    pub fn evaluate_expression(expression: &str) -> Result<f64, String> {
        match eval_str(expression) {
            Ok(result) => Ok(result),
            Err(_) => Err("Error: Invalid expression format".to_string()),
        }
    }

    // Function to log history
    pub fn log_history(entry: &str) {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("history.txt")
            .expect("Unable to open history file");
        writeln!(file, "{}", entry).expect("Unable to write to file");
    }

    // Function to read user input
    pub fn get_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // Flush output buffer for cleaner display
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string()
    }
}
