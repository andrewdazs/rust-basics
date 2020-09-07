use std::io; // io module

fn main() {
    let name = "Andrew";
    let another_name = "Edcarla";

    println!("{} and {}", name, another_name);

    let first = "Pascal".to_string();

    say_name(&first);
    say_name(&first);

    // Reading user input
    let mut second_name = String::new();
    println!("Enter your name: ");

    io::stdin().read_line(&mut second_name); // takes a mutable reference to the string buffer
    println!("Hello {}", second_name);
}

fn say_name(name: &String) {
    println!("{}", name);
}

fn sum()
