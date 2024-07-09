use std::io;

fn main() {
    println!("Hello, I do calculator functions with the basic operators. (+, -, *, /)");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to readline");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 3 {
            println!("invalid input");
            continue;
        }


        let a = match parts[0].parse::<f64>() {
            Ok(first_number) => { first_number }
            Err(error) => {
                println!("Failed to parse number: {}", error);
                continue;
            }
        };

        let b = match parts[2].parse::<f64>() {
            Ok(first_number) => { first_number }
            Err(error) => {
                println!("Failed to parse number: {}", error);
                continue;
            }
        };

        calculate(a, b, parts[1]);
    }
}

fn calculate(a: f64, b: f64, operator: &str) {
    match operator {
        "+" => println!("{}", add_two_numbers(a, b)),
        "-" => println!("{}", subtract_two_numbers(a, b)),
        "/" => {
            match divide_two_numbers(a, b) {
                Ok(value) => {
                    println!("{}", value)
                }
                Err(error) => {
                    println!("{}", error)
                }
            }
        }
        "*" => println!("{}", multiply_two_numbers(a, b)),

        _ => println!("Invalid operator")
    }
}

fn divide_two_numbers(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero error"))
    } else {
        Ok(a/b)
    }
}

fn multiply_two_numbers(a: f64, b: f64) -> f64 {
    a * b
}

fn subtract_two_numbers(a: f64, b: f64) -> f64 {
    a - b
}

fn add_two_numbers(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(add_two_numbers(2.0, 3.0), 5.0);
        assert_eq!(add_two_numbers(-2.0, 3.0), 1.0);
    }

    #[test]
    fn test_subtract_two_numbers() {
        assert_eq!(subtract_two_numbers(5.0, 3.0), 2.0);
        assert_eq!(subtract_two_numbers(3.0, 5.0), -2.0);
    }

    #[test]
    fn test_multiply_two_numbers() {
        assert_eq!(multiply_two_numbers(2.0, 3.0), 6.0);
        assert_eq!(multiply_two_numbers(-2.0, 3.0), -6.0);
    }

    #[test]
    fn test_divide_two_numbers() {
        assert_eq!(divide_two_numbers(6.0, 3.0).unwrap(), 2.0);
        assert_eq!(divide_two_numbers(6.0, -3.0).unwrap(), -2.0);
        assert!(divide_two_numbers(6.0, 0.0).is_err());
    }

    #[test]
    fn test_calculate() {

        let result = std::panic::catch_unwind(|| {
            calculate(2.0, 3.0, "+");
        });
        assert!(result.is_ok());
    }
}
