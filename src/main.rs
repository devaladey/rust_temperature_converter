use std::io;
fn main() {
    println!("Welcome to the temperature converter.");
    'outerloop: loop {
        message();

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to accept option");

        match option.trim() {
            "1" => loop {
                println!("Enter the celcius value:");

                let mut cel = String::new();

                io::stdin()
                    .read_line(&mut cel)
                    .expect("Failed to get cel value");

                let cel: i32 = match cel.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let farvalue = (cel as f64) * 9.0 / 5.0 + 32.0;
                println!("This is the farcius value: {farvalue}");
                break 'outerloop;
            },
            "2" => loop {
                println!("Enter the farenhiet value:");
                let mut far = String::new();

                io::stdin()
                    .read_line(&mut far)
                    .expect("Failed to get far value");

                let far: i32 = match far.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                let celvalue = (far as f64 - 32.0) * 5.0 / 9.0;
                println!("This is the celcius value: {celvalue}");
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
        "Select 1 to convert from farenheit to calcius.\nSelect 2 to convert from celcius to farenheit."
    );
}
