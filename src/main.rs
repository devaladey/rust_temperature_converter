use std::io;
fn main() {
    println!("Welcome to the temperature converter.");
    'outerloop: loop {
        message();

        let option = read_user_input("Option");

        match option.trim() {
            "1" => {
                let cel: i32 = read_int_until_valid("Celsius");
                let far_value = convert_to_farenheit(cel as f64);
                println!("This is the Fahrenheit value: {far_value}");
                break 'outerloop;
            },
            "2" => {
                let far = read_int_until_valid("Fahrenheit");
                let cel_value = convert_to_celsius(far as f64);
                println!("This is the Celsius value: {cel_value}");
                break 'outerloop;
            },
            _ => {
                println!("Invalid option.");
            }
        }
    }
}

fn message() {
    println!("What do you want to convert?");
    println!(
        "Select 1 to convert from fahrenheit to celsius.\nSelect 2 to convert from celsius to fahrenheit."
    );
}

fn read_user_input(unit: &str) -> String {
    println!("Enter the {unit} value:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect(&format!("Failed to get {unit} value"));

    input
}

fn parse_input_to_int(input: &str)-> Option<i32> {
    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn convert_to_farenheit(cel_val: f64)-> f64 {
    cel_val * 9.0 / 5.0 + 32.0
}

fn convert_to_celsius(far_val: f64)-> f64 {
  (far_val - 32.0) * 5.0 / 9.0  
}

fn read_int_until_valid(unit: &str)-> i32 {
    loop {
        let value = read_user_input(unit);

        if let Some(num) = parse_input_to_int(&value) {
            return num;
        }
        println!("Invalid number, try again.");
    }
}
