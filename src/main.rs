use std::io;

fn main() {
    println!("Hello, what's your name?");
    let your_name = read_string();
    let visitor_list = ["bert", "steve", "fred", "connor"];
    if visitor_list.contains(&your_name.as_str()) {
        println!("Welcome to the Treehouse, {}.", your_name);
    } else {
        println!("Sorry, you are not on the list.");
    }
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_lowercase()
}
