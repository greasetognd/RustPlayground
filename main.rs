
use std::io;
fn main() {
    println!("Enter the first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap(); // hai
    let num1: i32 = input1.trim().parse().unwrap();

    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let num2: i32 = input2.trim().parse().unwrap();

    let result = num1 * num2;

    println!();
    println!("{} x {} = {}", num1, num2, result);
    println!();
}
