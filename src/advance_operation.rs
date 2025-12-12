use crate::{get_input, confirm_retry};

fn read_number() -> Option<f64> {
    let input = get_input("Input Number: ");

    let number: Result<f64, _> = input.parse::<f64>();

    match number {
        Ok(nums) => Some(nums),
        Err(_) => {
            println!("Error: Input is not valid");
            None
        }
    }
}

fn advance_intro() {
    println!("==========================");
    println!("  ADVANCE MENU!  ");
    println!("1. Root (√)");
    println!("2. Log (log)");
    println!("3. Main Menu");
    println!("==========================");
    println!();
}

pub fn advance_menu() {
    'advance: loop {
        advance_intro();
        
        let advance_menu = get_input("Select Number Advance Menu: ");

        match advance_menu.as_str() {
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
        let number: f64 = read_number().unwrap();
        let get_root = get_input("Input Root (default: 2): ");

        let root: f64 = get_root.parse::<f64>().unwrap_or(2.0);

        let result = number.powf(1.0/root);

        println!("Result = {result}");
        println!();

        if !confirm_retry() { break 'root };
    }
}

fn log() {
    'log: loop {
        let get_log = get_input("Select Log Type 2 or 10 (default 10): ");
        let number: f64 = read_number().unwrap();

        let calculate_log: f64;

        match get_log.trim().to_lowercase().as_str() {
            "2" => calculate_log = number.log2(),
            _ => calculate_log = number.log10()
        };

        println!("Result = {calculate_log}");
        println!();

        if !confirm_retry() { break 'log };
    }
}