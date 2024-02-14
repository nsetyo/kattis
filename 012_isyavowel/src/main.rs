use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error read line");

    let vowels = input
        .trim()
        .chars()
        .filter(|c| ['a', 'i', 'u', 'e', 'o'].contains(c))
        .count();

    let y = input.trim().chars().filter(|&c| c == 'y').count();

    println!("{} {}", vowels, vowels + y);
}
