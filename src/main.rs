use std::io;
fn main() {
    println!("Welcome to the temperature converter.");
    message();

    loop {
        let option = read_option_until_valid();

        match option.trim() {
            "1" => {
                let cel = read_float_until_valid("Enter the Celsius value:");
                let far_value = convert_to_fahrenheit(cel);
                println!("This is the Fahrenheit value: {:.2}°F", far_value);
                if !continue_question() {
                    break;
                }
            }
            "2" => {
                let far = read_float_until_valid("Enter the Fahrenheit value:");
                let cel_value = convert_to_celsius(far);
                println!("This is the Celsius value: {:.2}°C", cel_value);
                if !continue_question() {
                    break;
                }
            }
            _ => {
                println!("Invalid option.");
            }
        }
    }
}

fn message() {
    println!("What do you want to convert?");
    println!(
        "Select 1 to convert from Fahrenheit to Celsius.\nSelect 2 to convert from Celsius to Fahrenheit."
    );
}

fn read_user_input(question: &str) -> String {
    println!("{question}");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect(&format!("Failed to read from input"));

    input
}

fn parse_input_to_float(input: &str) -> Option<f64> {
    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn convert_to_fahrenheit(cel_val: f64) -> f64 {
    cel_val * 9.0 / 5.0 + 32.0
}

fn convert_to_celsius(far_val: f64) -> f64 {
    (far_val - 32.0) * 5.0 / 9.0
}

fn read_float_until_valid(unit: &str) -> f64 {
    loop {
        let value = read_user_input(unit);

        if let Some(num) = parse_input_to_float(&value.trim()) {
            return num;
        }
        println!("Invalid number, try again.");
    }
}

fn continue_question() -> bool {
    let continue_text = read_user_input("Do you want to continue? (Y/N)");
    if continue_text.trim().eq_ignore_ascii_case("n") {
        return false;
    }

    true
}

fn read_option_until_valid() -> String {
    loop {
        let option = read_user_input("Enter Option value:");

        match option.trim() {
            "1" | "2" => return option.trim().to_string(),
            _ => println!("Invalid option, please select 1 or 2."),
        }
    }
}