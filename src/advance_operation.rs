use std::io;

fn advance_intro() {
    println!("==========================");
    println!("  ADVANCE MENU!  ");
    println!("1. Root");
    println!("2. Log");
    println!("3. Main Menu");
    println!("==========================");
    println!();
}

pub fn advance_menu() {
    'advance: loop {
        advance_intro();

        println!("Select Number Menu: ");
        let mut get_input = String::new();
        io::stdin()
            .read_line(&mut get_input)
            .expect("Failed togo run read line");
        println!();

        match get_input.trim().to_lowercase().as_str() {
            "1" => root(),
            "2" => log(),
            "3" => break 'advance,
            _ => {
                println!("Menu not found");
            }
        }
    }
}

fn root() {
    'root: loop {
        println!("Input Number: ");
        let mut get_number = String::new();
        io::stdin()
            .read_line(&mut get_number)
            .expect("Failed to read line");
        println!();
        let number: f32 = get_number.trim().parse::<f32>().unwrap();

        println!("Input Root (default: 2): ");
        let mut get_root = String::new();
        io::stdin()
            .read_line(&mut get_root)
            .expect("Failed to read line");
        println!();
        let root: f32 = get_root.trim().parse::<f32>().unwrap_or(2.0);

        let calculate_root = number.powf(1.0/root);


        println!("Result = {calculate_root}");
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
            break 'root;
        }
    }
}

fn log() {
    'log: loop {
        println!("Select Log Type 2 or 10 (default 10): ");
        let mut get_log = String::new();
        io::stdin()
            .read_line(&mut get_log)
            .expect("Failed to read line");
        println!();

        println!("Input Number: ");
        let mut get_number = String::new();
        io::stdin()
            .read_line(&mut get_number)
            .expect("Failed to read line");
        println!();

        let number = get_number.trim().parse::<f32>().unwrap();

        let calculate_log: f32;

        match get_log.trim().to_lowercase().as_str() {
            "2" => calculate_log = number.log2(),
            _ => calculate_log = number.log10()
        };

        println!("Result = {calculate_log}");
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
            break 'log;
        }
    }
}