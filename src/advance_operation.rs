use crate::{confirm_retry, get_input};

fn calc_root(number: f64, root: f64) -> Result<f64, &'static str> {
    if root == 0.0 {
        return Err("Cannot Process with root 0.0");
    }

    if number < 0.0 {
        return Err("Cannot process with negative number");
    }

    Ok(number.powf(1.0 / root))
}

fn calc_log(number: f64, base: f64) -> Result<f64, &'static str> {
    if number == 0.0 {
        return Err("Cannot Process with number 0.0");
    }

    if base <= 0.0 || base == 1.0 {
        return Err("Cannot Process with base is 1 or negative");
    }

    Ok(number.log(base))
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
            "1" => handle_root(),
            "2" => handle_log(),
            "3" => break 'advance,
            _ => {
                println!("Menu not found");
            }
        }
    }
}

fn handle_root() {
    loop {
        if let Some(number) = read_number() {
            let root_input = get_input("Input Root (default: 2): ");
            let root_val = if root_input.trim().is_empty() {
                2.0
            } else {
                root_input.trim().parse::<f64>().unwrap_or_else(|_| {
                    println!("Input not valid, using default root 2.0");
                    2.0
                })
            };

            match calc_root(number, root_val) {
                Ok(result) => println!("{result}"),
                Err(err) => println!("{err}"),
            }
        }

        if !confirm_retry() {
            break;
        }
    }
}

fn handle_log() {
    loop {
        if let Some(number) = read_number() {
            let base_input = get_input("Input Base (Default 10): ");

            let base_val = if base_input.trim().is_empty() {
                10.0
            } else {
                base_input.trim().parse::<f64>().unwrap_or_else(|_| 10.0)
            };

            match calc_log(number, base_val) {
                Ok(result) => println!("{result}"),
                Err(err) => println!("{err}"),
            }
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
    fn test_root() {
        assert_eq!(calc_root(81.0, 2.0), Ok(9.0));
        assert_eq!(calc_root(81.0, 0.0), Err("Cannot Process with root 0.0"));
        assert_eq!(calc_root(0.0, 2.0), Ok(0.0));
        assert_eq!(calc_root(0.0, 0.0), Err("Cannot Process with root 0.0"));
        assert_eq!(calc_root(-1.0, 2.0), Err("Cannot process with negative number"));
    }
    
    #[test]
    fn test_log() {
        assert_eq!(calc_log(100.0, 10.0), Ok(2.0));
        assert_eq!(calc_log(0.0, 10.0), Err("Cannot Process with number 0.0"));
        assert_eq!(calc_log(100.0, 0.0), Err("Cannot Process with base is 1 or negative"));
        assert_eq!(calc_log(100.0, 1.0), Err("Cannot Process with base is 1 or negative"));
        assert_eq!(calc_log(100.0, -1.0), Err("Cannot Process with base is 1 or negative"));
    }
}