How to create a new project in Rust:
  - cargo new project-name
  - Cargo.toml -> its the package file.

Compile and run:
  - cargo build
  - generate an executable file in target/debug/project-name
  - and just run it: target/debug/example-project -> output: "Hello, world!"
  - OR just run: cargo run

Clean the target:
  - cargo clean -> removes the generated target files.

Variables:
  - Are immutable by default. You can't override them.
  - You can use "mut" to make the variable mutable. Like: let mut name = "Andrew";

Output multiple variables:
  - println!("{} and {}", name, another_name);

Ownership and Borrowing:
  - You can't call the same function twice, because when we call a function
  we are passing the variable value and not the variable reference into it,
  and because of that the variable is lo longer available. Like this:
    my_function(name);
    my_function(name); // the parameter "name" is no longer available;

  - So, to solve this, we need to pass the parameter as reference and not as value. Like this:
    my_function(&name); -> // we are saying: pass the reference for me, and not the value.
    my_function(&name); // the parameter "name" is no longer available;

    fn my_function(name: &String) { -> now we're passing the string reference
    }

Reading user input:
  - Use stdin

Handle errors with:
  - `unwrap()` -> do not use it in production.
  - `expect()`
  - Pattern Matching
    - Use Ok and Err functions to handle it.

Exit a program using std::process
  - process::exit(1) -> if not 0 means that the program exit with an error.

Loops:
  - just wrap the the code with the "loop" function. Like this:
    loop {
      // our code
    }