use std::io;

fn main() {
    println!("Enter 1 to convert from Celsius to Fahrenheit, or 2 to convert from Fahrenheit to Celsius.");
    let mut choice: String = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = choice.trim().parse().expect("Please type a number!");

    if choice == 1 {
        celsius();
    } else {
        fahrenheit();
    }
}

fn celsius() {
    println!("Please enter the temperature in Celsius: ");
    let mut celsius: String = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");

    let celsius: u32 = celsius.trim().parse().expect("Please type a number!");

    let fahrenheit: u32 = (celsius * 9/5) + 32;

    println!("The temperature in Fahrenheit is: {}", fahrenheit);
}

fn fahrenheit() {
    println!("Please enter the temperature in Fahrenheit: ");
    let mut fahrenheit: String = String::new();

    io::stdin()
    .read_line(&mut fahrenheit)
    .expect("Failed to read line");

    let fahrenheit: u32 = fahrenheit.trim().parse().expect("Please type a number!");

    let celsius: u32 = (fahrenheit - 32) * 5/9;

    println!("The temperature in Celsius is: {}", celsius)
}