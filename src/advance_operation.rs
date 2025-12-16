use crate::{get_input, confirm_retry};

fn calc_root(number: &f64) -> f64 {
    let get_root = get_input("Input Root (default: 2): ");
    let root: f64 = get_root.parse::<f64>().unwrap_or(2.0);

    number.powf(1.0/root)
}

fn calc_log(number: &f64) -> f64 {
    let get_log = get_input("Select Log Type 2 or 10 (default 10): ");

    match get_log.trim().to_lowercase().as_str() {
        "2" => number.log2(),
        _ => number.log10()
    }
}

fn read_number() -> Option<f64> {
    let input = get_input("Input Number: ");

    let number: Result<f64, _> = input.parse::<f64>();

    match number {
        Ok(num) => Some(num),
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
            "1" => run_operation("Root", |num| Ok(calc_root(num))),
            "2" => run_operation("Log", |num| Ok(calc_log(num))),
            "3" => break 'advance,
            _ => {
                println!("Menu not found");
            }
        }
    }
}

fn run_operation<F>(name: &str, op: F)
where
    F: Fn(&f64) -> Result<f64, &'static str>
{
    loop {
        if let Some(number) = read_number() {
            match op(&number) {
                Ok(result) => println!("{} = {}", name, result),
                Err(e) => println!("Error = {}", e),
            }
            println!();
        }

        if !confirm_retry() { break; }
    }
}