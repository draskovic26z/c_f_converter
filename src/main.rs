use std::io;

fn main() {
    println!("\n Cº <=> Fº (by: zazz)\n");

    'main_loop: loop {
        //choosing which unit
        println!("\nPick the starting unit, F or C:");
        loop {
            let mut first_unit: String = String::new();
            io::stdin()
                .read_line(&mut first_unit)
                .expect("Failed to read line.");

            //calls appropriate functions
            match first_unit.trim() {
                "F" => {
                    f_path();
                    break;
                }
                "f" => {
                    f_path();
                    break;
                }
                "C" => {
                    c_path();
                    break;
                }
                "c" => {
                    c_path();
                    break;
                }
                _ => {
                    println!("Please type only F or C!");
                    continue;
                }
            }
        }
        println!("\nDo you want to convert again? (type y or n)");
        loop {
            let mut again: String = String::new();

            io::stdin()
                .read_line(&mut again)
                .expect("\nFailed to read line.");

            match again.trim().parse::<char>() {
                Ok(char) => match char {
                    'y' => continue 'main_loop,
                    'n' => {
                        println!("Bye!");
                        break 'main_loop;
                    }
                    _ => {
                        println!("\nPlease type only y or n!");
                        continue;
                    }
                },
                Err(_) => {
                    println!("\nPlease type only y or n!");
                    continue;
                }
            };
        }
    }
}

fn f_path() {
    loop {
        println!("\nPlease input the temperature in Fahrenheit:");

        let mut value: String = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read the Fahrenheit value!");

        match value.trim().parse::<i32>() {
            Ok(num) => {
                let converted: i32 = (num - 32) * 5 / 9;
                println!("\n{num}ºF => {converted}ºC");
                break;
            }
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
    }
}

fn c_path() {
    loop {
        println!("\nPlease input the temperature in Celsius:");

        let mut value: String = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read the Celsius value!");

        match value.trim().parse::<i32>() {
            Ok(num) => {
                let converted: i32 = (num * 9 / 5) + 32;
                println!("\n{num}ºC => {converted}ºF");
                break;
            }
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
    }
}
