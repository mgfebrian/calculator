use basic_operation::basic_menu;
use advance_operation::advance_menu;
use std::io::{self, Write};

pub mod basic_operation;
pub mod advance_operation;

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)
        .expect("Failed get input");

    println!();

    buffer.trim().to_string()
}

fn confirm_retry() -> bool {
    let response = get_input("Do you want to try again? (Y/n) ");
    matches!(response.to_lowercase().as_str(), "y" | "")
}

fn introduction() {
    println!("==========================");
    println!("  WELCOME TO CALCULATOR!  ");
    println!("1. Help");
    println!("2. Basic Tools");
    println!("3. Advance Tools");
    println!("4. History");
    println!("5. Exit");
    println!("==========================");
    println!();
}

fn help_menu() {
    println!("==========================");
    println!("  HELP MENU  ");
    println!("1. Help = information menu");
    println!("2. Basic Tools = operation basic (Add, Sub, Mul, Div, Mod)");
    println!("3. Advance Tools = operation advance (Pow, Sqrt, Log, Abs, Round)");
    println!("4. History = History tools usage");
    println!("5. Exit = Exit Calculator");
    println!("==========================");
    println!();
}

fn main() {
    'intro: loop {
        introduction();
        
        let main_menu = get_input("Select Number Main Menu: ");

        match main_menu.as_str() {
            "1" => help_menu(),
            "2" => basic_menu(),
            "3" => advance_menu(),
            "4" => println!("1"),
            "5" => {
                println!("Bye Bye");
                break 'intro;
            }
            _ => {
                println!("Menu not found");
            }
        }
    }
}
