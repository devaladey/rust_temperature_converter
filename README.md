# Rust Temperature Converter 🌡️

A simple terminal-based temperature converter written in Rust. Convert between Celsius and Fahrenheit with ease, and the program allows you to continue converting multiple times.

## Features

* Convert Celsius to Fahrenheit and vice versa.
* Input validation with floating-point support.
* Continuous conversions until you choose to exit.
* Nicely formatted output with 2 decimal precision.

## Installation & Running

1. Make sure you have Rust installed.
2. Clone this repository and navigate into it:

```bash
git clone https://github.com/your-username/rust-temperature-converter.git
cd rust-temperature-converter
```

3. Build and run the program with Cargo:

```bash
cargo run
```

This will compile and start the converter in your terminal.

## How to Use

1. After running the program, you'll be asked to choose an option:

   * Enter `1` to convert Celsius to Fahrenheit.
   * Enter `2` to convert Fahrenheit to Celsius.
2. Input the temperature value you want to convert.
3. The program will display the converted value with 2 decimal precision.
4. You'll be asked if you want to continue. Enter `Y` for yes or `N` for no.

## Example

```
Welcome to the temperature converter.
What do you want to convert?
Select 1 to convert from Fahrenheit to Celsius.
Select 2 to convert from Celsius to Fahrenheit.
Enter Option value: 1
Enter the Celsius value: 25
This is the Fahrenheit value: 77.00°F
Do you want to continue? (Y/N) N
```

## Contributing

Feel free to fork the repository and submit improvements, such as:

* Adding Kelvin conversion.
* Implementing more input error handling.
* Supporting batch conversions.

## License

This project is licensed under the MIT License.
