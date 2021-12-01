use std::io;

fn main() {
    println!("Hello, what's your name?");
    let your_name = read_string();
    println!("Hello, {}!", your_name);
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    String::from(input.trim())
}
