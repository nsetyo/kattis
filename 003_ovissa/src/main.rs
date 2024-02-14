use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error read the line");

    let output: String = input.chars().filter(|&c| c == 'u').collect();

    println!("{}", output.len());
}
