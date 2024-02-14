use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    let output: String = input.trim().chars().filter(|c| c.is_uppercase()).collect();

    println!("{}", output);
}
