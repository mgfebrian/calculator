use basic_operation::basic_menu;
use advance_operation::advance_menu;

pub mod basic_operation;
pub mod advance_operation;

use std::io;

// enum Basic {
//     Add,
//     Sub,
//     Mul,
//     Div,
//     Mod,
// }
//
// enum Advance {
//     Pow,
//     Sqrt,
//     Log,
//     Abs,
//     Round
// }

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

fn get_input() -> String {
    let mut get_input = String::new();
    io::stdin()
        .read_line(&mut get_input)
        .expect("Failed to read line");

    get_input
}

fn main() {
    'intro: loop {
        introduction();

        println!("Select Number Menu: ");
        let get_menu = get_input();
        println!();

        match get_menu.trim().to_lowercase().as_str() {
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
