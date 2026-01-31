# Rust Temperature Converter 🌡️

A simple temperature conversion program written in **Rust** that runs in the terminal.
The program allows users to convert temperatures between **Celsius and Fahrenheit** using a menu-driven interface with proper input validation.

---

## Features

* Convert **Celsius → Fahrenheit**
* Convert **Fahrenheit → Celsius**
* Menu-based user interaction
* Handles invalid menu input
* Handles invalid numeric input safely
* Uses floating-point arithmetic for accurate results
* Beginner-friendly Rust code

---

## Installation & Running

### Prerequisites

Make sure you have **Rust** installed.

Check by running:

```bash
rustc --version
```

If Rust is not installed, download it from:
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

---

### Clone and Run

```bash
git clone https://github.com/your-username/rust-temperature-converter.git
cd rust-temperature-converter
cargo run
```

This command compiles the program and runs it in your terminal.

---

## How to Use

1. Run the program.
2. Choose an option from the menu:

   * `1` → Convert from **Celsius to Fahrenheit**
   * `2` → Convert from **Fahrenheit to Celsius**
3. Enter the temperature value when prompted.
4. The converted temperature will be displayed.
5. The program exits after a successful conversion.

---

## Example

```text
Welcome to the temperature converter.
What do you want to convert?
Select 1 to convert from farenheit to calcius.
Select 2 to convert from celcius to farenheit.

1
Enter the celcius value:
100
This is the farcius value: 212
```

```text
2
Enter the farenhiet value:
32
This is the celcius value: 0
```

---

## Learning Goals

This project demonstrates:

* Reading user input from the terminal
* Using `match` statements in Rust
* Handling errors with `Result`
* Loop control using labeled loops
* Converting between numeric types
* Floating-point arithmetic in Rust

---

## Contributing

Contributions are welcome!
If you’d like to improve this project, feel free to fork the repository and submit a pull request.

Possible improvements:

* Allow multiple conversions per run
* Add support for Kelvin
* Improve user interface messages
* Add unit tests

---

## License

This project is licensed under the **MIT License**.
