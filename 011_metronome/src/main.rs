use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error read line");

    let input: f32 = input.trim().parse().expect("Invalid number");

    let output = format!("{:.2}", input / 4.0);

    let output = match output.strip_suffix('0') {
        Some(s) => s.to_owned(),
        None => output,
    };

    println!("{output}");
}
