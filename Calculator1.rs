use std::io;

fn main() {
    loop {
        let mut input = String::new();

        println!("Enter the first number:");
        io::stdin().read_line(&mut input).unwrap();
        let f: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid integer.");
                continue;
            }
        };

        input.clear();
        println!("Enter the second number:");
        io::stdin().read_line(&mut input).unwrap();
        let s: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid integer.");
                continue;
            }
        };

        println!(
            "Choose an operation:\n\
            ADD +\n\
            SUB -\n\
            MULTIPLY *\n\
            DIVIDE /\n\
            FLOOR DIVISION //\n\
            EXPONENT **\n\
            MODULUS %"
        );

        input.clear();
        println!("Enter your choice:");
        io::stdin().read_line(&mut input).unwrap();
        let ch = input.trim();

        match ch {
            "ADD" | "add" | "+" => println!("Sum: {} + {} = {}", f, s, f + s),
            "SUB" | "sub" | "-" => println!("Difference: {} - {} = {}", f, s, f - s),
            "MULTIPLY" | "multiply" | "*" => println!("Product: {} * {} = {}", f, s, f * s),
            "DIVIDE" | "divide" | "/" => {
                if s == 0 {
                    println!("Error: Division by zero is not allowed.");
                } else {
                    println!("Quotient: {} / {} = {:.2}", f, s, f as f64 / s as f64);
                }
            }
            "FLOOR" | "floor" | "//" => {
                if s == 0 {
                    println!("Error: Division by zero is not allowed.");
                } else {
                    println!("Floor Division: {} // {} = {}", f, s, f / s);
                }
            }
            "EXPONENT" | "exponent" | "**" => {
                if s < 0 {
                    println!("Negative exponents are not supported for integers.");
                } else {
                    println!("Exponentiation: {} ** {} = {}", f, s, f.pow(s as u32));
                }
            }
            "MOD" | "mod" | "%" => {
                if s == 0 {
                    println!("Error: Modulo by zero is not allowed.");
                } else {
                    println!("Modulus: {} % {} = {}", f, s, f % s);
                }
            }
            _ => println!("Invalid choice. Please try again."),
        }

        input.clear();
        println!("Do you want to continue? [Y/N]:");
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().eq_ignore_ascii_case("N") {
            break;
        }
    }
}
