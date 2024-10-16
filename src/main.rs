use std::io::{self, Write};

fn main() {
    let mut result: f64 = 0.0;

    loop {
        print!("Введіть операцію (add, sub, mul, div) або 'exit' для виходу: ");
        io::stdout().flush().unwrap();

        let mut operation = String::new();
        io::stdin().read_line(&mut operation).unwrap();

        let operation = operation.trim();

        if operation == "exit" {
            break;
        }

        print!("Введіть перше число: ");
        io::stdout().flush().unwrap();

        let mut first_num = String::new();
        io::stdin().read_line(&mut first_num).unwrap();

        let first_num: f64 = match first_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Помилка: Некоректне число.");
                continue;
            }
        };

        print!("Введіть друге число: ");
        io::stdout().flush().unwrap();

        let mut second_num = String::new();
        io::stdin().read_line(&mut second_num).unwrap();

        let second_num: f64 = match second_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Помилка: Некоректне число.");
                continue;
            }
        };

        result = match operation {
            "add" => first_num + second_num,
            "sub" => first_num - second_num,
            "mul" => first_num * second_num,
            "div" => {
                if second_num == 0.0 {
                    println!("Помилка: Ділення на нуль.");
                    continue;
                }
                first_num / second_num
            }
            _ => {
                println!("Помилка: Некоректна операція.");
                continue;
            }
        };

        println!("Результат: {}", result);
    }
}

