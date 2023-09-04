use std::io;

fn main() {
    println!("Enter the nth number of the Fibonacci sequence you want to calculate:");
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = input.trim().parse().expect("Please type a number!");

    println!("The {}th number of the Fibonacci sequence is {}", n, calculate_fibonacci(n));
}

fn calculate_fibonacci(n: u32) -> u32 {
    if (n == 0) || (n == 1) {
        return n;
    } else {
        return calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2);
    }


}
