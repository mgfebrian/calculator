use std::io;

fn basic_intro() {
    println!("==========================");
    println!("  BASIC MENU!  ");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("5. Module");
    println!("6. Main Menu");
    println!("==========================");
    println!();
}

pub fn basic_menu() {
    'basic: loop {
        basic_intro();

        println!("Select Number Menu: ");
        let mut get_input = String::new();
        io::stdin()
            .read_line(&mut get_input)
            .expect("Failed to read line");
        println!();

        match get_input.trim().to_lowercase().as_str() {
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
        println!("Input Number and separate with whitespace: ");
        let mut get_input = String::new();
        io::stdin()
            .read_line(&mut get_input)
            .expect("Failed to read line");
        println!();

        let numbers: Vec<i32> = get_input
            .clone()
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let calculate_numbers: i32 = numbers.iter().sum();

        println!("Addition = {calculate_numbers}");
        println!();

        println!("Try Operation Again? (Y/n) ");
        let mut loop_operation = String::new();
        io::stdin()
            .read_line(&mut loop_operation)
            .expect("Failed to read line");
        println!();

        let is_loop: bool;

        match loop_operation.trim().to_lowercase().as_str() {
            "y" => is_loop = true,
            "n" => is_loop = false,
            _ => is_loop = true
        }

        if !is_loop {
            break 'addition;
        }
    }
}

fn substraction() {
    'substraction: loop {
        println!("Input Number and separate with whitespace: ");
        let mut get_input = String::new();
        io::stdin()
            .read_line(&mut get_input)
            .expect("Failed to read line");

        let mut numbers: Vec<i32> = get_input
            .clone()
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut initial_substraction: i32 = numbers.remove(0);

        for x in numbers {
            initial_substraction -= x;
        }

        println!("Subsctraction = {initial_substraction}");


        println!("Try Operation Again? (Y/n) ");
        let mut loop_operation = String::new();
        io::stdin()
            .read_line(&mut loop_operation)
            .expect("Failed to read line");
        println!();

        let is_loop: bool;

        match loop_operation.trim().to_lowercase().as_str() {
            "y" => is_loop = true,
            "n" => is_loop = false,
            _ => is_loop = true
        }

        if !is_loop {
            break 'substraction;
        }
    }
}

fn multiplication() {
    'multiplication: loop {
        println!("Input Number and separate with whitespace: ");
        let mut get_input = String::new();
        io::stdin()
            .read_line(&mut get_input)
            .expect("Failed to read line");
        println!();

        let numbers: Vec<i32> = get_input
            .clone()
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let calculate_numbers: i32 = numbers.iter().product();

        println!("Addition = {calculate_numbers}");
        println!();

        println!("Try Operation Again? (Y/n) ");
        let mut loop_operation = String::new();
        io::stdin()
            .read_line(&mut loop_operation)
            .expect("Failed to read line");
        println!();

        let is_loop: bool;

        match loop_operation.trim().to_lowercase().as_str() {
            "y" => is_loop = true,
            "n" => is_loop = false,
            _ => is_loop = true
        }

        if !is_loop {
            break 'multiplication;
        }
    }
}

fn division() {
    'division: loop {
        println!("Input Number and separate with whitespace: ");
        let mut get_input = String::new();
        io::stdin()
            .read_line(&mut get_input)
            .expect("Failed to read line");

        let mut numbers: Vec<f32> = get_input
            .clone()
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<f32>().unwrap())
            .collect();

        let mut initial_division: f32 = numbers.remove(0);

        for x in numbers {
            initial_division /= x;
        }

        println!("Subsctraction = {initial_division}");


        println!("Try Operation Again? (Y/n) ");
        let mut loop_operation = String::new();
        io::stdin()
            .read_line(&mut loop_operation)
            .expect("Failed to read line");
        println!();

        let is_loop: bool;

        match loop_operation.trim().to_lowercase().as_str() {
            "y" => is_loop = true,
            "n" => is_loop = false,
            _ => is_loop = true
        }

        if !is_loop {
            break 'division;
        }
    }
}

fn module() {
    'module: loop {
        println!("Input Number and separate with whitespace: ");
        let mut get_input = String::new();
        io::stdin()
            .read_line(&mut get_input)
            .expect("Failed to read line");

        let mut numbers: Vec<f32> = get_input
            .clone()
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<f32>().unwrap())
            .collect();

        let mut initial_module: f32 = numbers.remove(0);

        for x in numbers {
            initial_module %= x;
        }

        println!("Subsctraction = {initial_module}");


        println!("Try Operation Again? (Y/n) ");
        let mut loop_operation = String::new();
        io::stdin()
            .read_line(&mut loop_operation)
            .expect("Failed to read line");
        println!();

        let is_loop: bool;

        match loop_operation.trim().to_lowercase().as_str() {
            "y" => is_loop = true,
            "n" => is_loop = false,
            _ => is_loop = true
        }

        if !is_loop {
            break 'module;
        }
    }
}