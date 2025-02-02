mod calculator; // Import the calculator module

use calculator::Calculator; // Bring the Calculator struct into scope

fn main() {
    loop {
        let mode = Calculator::get_input("Do you want to do an inline calculation or evaluate an expression? (inline/express): ");

        match mode.to_lowercase().as_str() {
            "inline" => {
                // Inline calculation mode
                let num1: f64 = match Calculator::get_input("Enter first number: ").parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid number.");
                        continue;
                    }
                };

                let operator = Calculator::get_input("Enter operation (+, -, *, /, %, ^, fact, sqrt): ");

                let num2 = if operator != "fact" && operator != "sqrt" {
                    match Calculator::get_input("Enter second number: ").parse() {
                        Ok(n) => Some(n),
                        Err(_) => {
                            println!("Invalid input. Please enter a valid number.");
                            continue;
                        }
                    }
                } else {
                    None
                };

                match Calculator::calculate(num1, &operator, num2) {
                    Ok(result) => {
                        let history_entry = format!("{} {} {} = {}", num1, operator, num2.unwrap_or(0.0), result);
                        println!("Result: {}", result);
                        Calculator::log_history(&history_entry);
                    }
                    Err(e) => println!("{}", e),
                }
            }
            "express" => {
                // Expression evaluation mode
                let expression = Calculator::get_input("Enter an expression (e.g., 10 + 2 * 3): ");
                match Calculator::evaluate_expression(&expression) {
                    Ok(result) => {
                        println!("Result: {}", result);
                        Calculator::log_history(&expression);
                    }
                    Err(e) => println!("{}", e),
                }
            }
            _ => {
                println!("Invalid option. Please choose 'inline' or 'express'.");
                continue;
            }
        }

        if Calculator::get_input("Do you want to perform another calculation? (yes/no): ").to_lowercase() != "yes" {
            break;
        }
    }
}
