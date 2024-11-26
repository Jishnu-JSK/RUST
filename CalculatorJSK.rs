use std::io;
fn calculator()
{
  loop 
    {
    let mut input=String::new();
    println!("Enter an operator (ADD,SUB,MULTIPLY,DIVIDE,MODULUS,POWER,ROOT,EXIT): ");
    io::stdin().read_line(&mut input).unwrap();
    let operator=input.trim().to_lowercase();
    input.clear();
    if operator == "exit" {
        println!("Exiting the Calculator!");
        break;
    }
    println!("Enter first number: ");
    io::stdin().read_line(&mut input).unwrap();
    let a:f64 =input.trim().parse().expect("!!Please enter a valid number!!");
    input.clear();
    println!("Enter second number: ");
    io::stdin().read_line(&mut input).unwrap();
    let b:f64=input.trim().parse().expect("!!Please enter a valid number!! ");
    input.clear();
    if operator=="add"||operator=="+"{
      println!("Sum: {}",a+b);
    }
    else if operator=="sub"||operator=="-"{
      println!("Difference: {}",a-b);
      }
    else if operator=="multiply"||operator=="*"{
      println!("Product: {}",a*b);
      }
    else if operator == "divide"||operator == "/"{
      if b==0.0{
        println!("Division by zero is not allowed!!");
        }
      else{
        println!("Quotient: {}",a/b);
        }
      }
    else if operator == "modulus" || operator == "%"{
      println!("Remainder: {}",a%b);
      }
    else if operator == "power"||operator == "^"{
      println!("Power:{}",a.powf(b));
      }
    else if operator == "root"{
      println!("Root of {} is {}",a,a.sqrt());
      println!("Root of {} is {}",b,b.sqrt());
      }
    }
  }
fn main(){
  calculator()
}