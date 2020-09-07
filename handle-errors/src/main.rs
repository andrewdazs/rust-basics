use std::io;
use std::process;
/* 
/ ERROR HANDLING METHODS
/ unwrap() -> but do not use it in production.
/ expect("This is not a valid number");
/ match -> handle success and fail cases.
*/

fn main() {
    println!("Enter the first number: ");
    let a = read_user_input();

    println!("Enter the second number: ");
    let b = read_user_input();

    let result = sum(a, b);
    println!("{} + {} = {}", a, b, result);
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

fn read_user_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let digit:u32;

    match input.trim().parse() {
        Ok(val) => {
            digit = val;
        },
        Err(_err) => {
            println!("{}", _err);
            process::exit(1);
        }
    }
    return digit;
}