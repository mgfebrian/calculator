use crate::{get_input, confirm_retry};

fn read_numbers() -> Option<Vec<f64>> {
    let input = get_input("Input Number and separate with whitespace: ");

    let numbers: Result<Vec<f64>, _> = input
        .split_whitespace()
        .map(|x| x.parse::<f64>())
        .collect();

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
            "1" => addition(),
            "2" => substraction(),
            "3" => multiplication(),
            "4" => division(),
            "5" => module(),
            "6" => break 'basic,
            _ => {
                println!("Menu not found");
            }
        }
    }
}

fn addition() {
    'addition: loop {
        let numbers = read_numbers().unwrap();
        let result: f64 = numbers.iter().sum();

        println!("Addition = {result}");
        println!();
        
        if !confirm_retry() { break 'addition };
    }
}

fn substraction() {
    'substraction: loop {
        let numbers = read_numbers().unwrap();
        let result = numbers.iter().skip(1).fold(numbers[0], |acc, &x| acc - x);

        println!("Subsctraction = {result}");
        println!();

        if !confirm_retry() { break 'substraction };
    }
}

fn multiplication() {
    'multiplication: loop {
        let numbers = read_numbers().unwrap();
        let result: f64 = numbers.iter().product();

        println!("multiplication = {result}");
        println!();

        if !confirm_retry() { break 'multiplication };
    }
}

fn division() {
    'division: loop {
        let numbers = read_numbers().unwrap();

        if numbers.contains(&0.0) {
            println!("Error: Division Must not Contain 0");
        } else {
            let result = numbers.iter().skip(1).fold(numbers[0], |acc, &x| acc / x);

            println!("division = {result}");
            println!();
        }

        if !confirm_retry() { break 'division };
    }
}

fn module() {
    'module: loop {
        let numbers = read_numbers().unwrap();

        if numbers.contains(&0.0) {
            println!("Error: Module Must not Contain 0");
        } else {
            let result = numbers.iter().skip(1).fold(numbers[0], |acc, &x| acc % x);

            println!("Module = {result}");
            println!();
        }
        
        if !confirm_retry() { break 'module };
    }
}