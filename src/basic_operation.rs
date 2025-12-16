use crate::{confirm_retry, get_input};

pub fn calc_addition(numbers: &[f64]) -> f64 {
    numbers.iter().sum()
}

pub fn calc_substraction(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    numbers.iter().skip(1).fold(numbers[0], |acc, &x| acc - x)
}

pub fn calc_multiplication(numbers: &[f64]) -> f64 {
    numbers.iter().product()
}

pub fn calc_division(numbers: &[f64]) -> Result<f64, &'static str> {
    if numbers.is_empty() {
        return Err("Input is empty");
    }

    if numbers.iter().skip(1).any(|&x| x == 0.0) {
        return Err("Cannot divide with 0.0");
    }

    Ok(numbers.iter().skip(1).fold(numbers[0], |acc, &x| acc / x))
}

pub fn calc_module(numbers: &[f64]) -> Result<f64, &'static str> {
    if numbers.is_empty() {
        return Err("Input is empty");
    }

    if numbers.iter().skip(1).any(|&x| x == 0.0) {
        return Err("Cannot modulo with 0.0");
    }

    Ok(numbers.iter().skip(1).fold(numbers[0], |acc, &x| acc % x))
}

fn read_numbers() -> Option<Vec<f64>> {
    let input = get_input("Input Number and separate with whitespace: ");

    let numbers: Result<Vec<f64>, _> = input.split_whitespace().map(|x| x.parse::<f64>()).collect();

    match numbers {
        Ok(nums) if nums.is_empty() => {
            println!("Error: Input is empty");
            None
        }
        Ok(nums) => Some(nums),
        Err(_) => {
            println!("Error: Input is not valid");
            None
        }
    }
}

fn basic_intro() {
    println!("==========================");
    println!("  BASIC MENU!  ");
    println!("1. Addition (+)");
    println!("2. Subtraction (-)");
    println!("3. Multiplication (*)");
    println!("4. Division (/)");
    println!("5. Module (%)");
    println!("6. Main Menu");
    println!("==========================");
    println!();
}

pub fn basic_menu() {
    'basic: loop {
        basic_intro();

        let basic_menu = get_input("Select Number Basic Menu: ");

        match basic_menu.as_str() {
            "1" => run_operation("Addition", |nums| Ok(calc_addition(nums))),
            "2" => run_operation("Substraction", |nums| Ok(calc_substraction(nums))),
            "3" => run_operation("Multiplication", |nums| Ok(calc_multiplication(nums))),
            "4" => run_operation("Division", calc_division),
            "5" => run_operation("Module", calc_module),
            "6" => break 'basic,
            _ => println!("Menu not found"),
        }
    }
}

fn run_operation<F>(name: &str, op: F)
where
    F: Fn(&[f64]) -> Result<f64, &'static str>,
{
    loop {
        if let Some(numbers) = read_numbers() {
            match op(&numbers) {
                Ok(result) => println!("{} = {}", name, result),
                Err(e) => println!("Error = {}", e),
            }
            println!();
        }
        if !confirm_retry() {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calc_addition(&[1.0, 2.0, 3.0]), 6.0);
        assert_eq!(calc_addition(&[]), 0.0);
    }

    #[test]
    fn test_substraction() {
        assert_eq!(calc_substraction(&[3.0, 2.0]), 1.0);
        assert_eq!(calc_substraction(&[]), 0.0);
    }
    #[test]
    fn test_multiplication() {
        assert_eq!(calc_multiplication(&[3.0, 2.0]), 6.0);
        assert_eq!(calc_multiplication(&[]), 1.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(calc_division(&[4.0, 2.0]), Ok(2.0));
        assert_eq!(calc_division(&[0.0]), Ok(0.0));
        assert_eq!(calc_division(&[4.0, 0.0]), Err("Cannot divide with 0.0"));
        assert_eq!(calc_division(&[]), Err("Input is empty"));
    }

    #[test]
    fn test_module() {
        assert_eq!(calc_module(&[9.0, 3.0]), Ok(0.0));
        assert_eq!(calc_module(&[0.0]), Ok(0.0));
        assert_eq!(calc_module(&[9.0, 0.0]), Err("Cannot modulo with 0.0"));
        assert_eq!(calc_module(&[]), Err("Input is empty"));
    }
}
