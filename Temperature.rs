use std::io;
fn temperature()
{
    loop 
    {
        let mut input=String::new();
        println!("Enter Temperature: ");
        io::stdin().read_line(&mut input).unwrap();
        let t:f64 =input.trim().parse().expect("Failed to read the Input!!");
        println!("Temperature:{}",t);
        input.clear();

        println!("Enter the unit of the Temperature:(C,F,K)");
        io::stdin().read_line(&mut input).unwrap();
        let unit = input.trim().to_uppercase();
        input.clear();

        println!("What unit do u want to convert your given temperature value:");
        io::stdin().read_line(&mut input).unwrap();
        let convert = input.trim().to_uppercase();
        input.clear();

        if unit == "C" {
            if convert == "F" {
                println!("Temperature is converted to Fahrenheit");
                println!("{}C = {}F",t,(t*9.0/5.0)+32.0);
            }
            else if convert == "K" {
                println!("Temperature is converted to Kelvin");
                println!("{}C = {}K",t,t+273.15);
            }
        }
        else if unit == "F" {
            if convert == "C" {
                println!("Temperature is converted to Celsius");
                println!("{}F = {}C",t,(t-32.0)*5.0/9.0);
            }
            else if convert == "K" {
                println!("Temperature is converted to Kelvin");
                println!("{}F = {}K",t,(t-32.0)*(5.0/9.0)+273.15);
            }
        }
        else if unit == "K" {
            if convert == "C" {
                println!("Temperature is converted to Celsius");
                println!("{}K = {}C",t,t-273.15);
            }
            else if convert == "F" {
                println!("Temperature is converted to Fahrenheit");
                println!("{}K = {}F",t,(t-273.15)*(9.0/5.0)+32.0)
            }
        }

        println!("Do you want to perform more conversion? (y/n)");
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_lowercase();
        input.clear();

        if choice == "n" {
            println!("Goodbye!");
            break;
        }
    }
}
fn main()
{
    temperature()
}

