use calculator_02::{calculate, read_line};

fn main() {
    println!("Welcome to the calculator!");
    loop {
        println!("Enter an expression to calculate (e.g. 2 + 2): !(Use space between entered data)");
        let input = match read_line() {
            Ok(value) => value,
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        };

        let trimmed_input = input.trim();
        if trimmed_input == "exit" {
            println!("Goodbye!");
            break;
        }
        println!("You entered = \"{trimmed_input}\"");
        let result = calculate(trimmed_input);
        println!("Result: {result}", );
    }
}

