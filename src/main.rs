use std::{fs, io::{self, Read}};

fn read_file(contents: &str) -> Vec<String> {
    contents 
    .split("\n")
    .map(|line| {
        line.to_string()
    })
    .collect::<Vec<String>>()
    .windows(2)
    .enumerate()
    .filter(|p| {p.0 % 2 == 0})
    .map(|(_, vec)| {
        format!("--{}: {};", vec[0], vec[1])
    })
    .collect::<Vec<String>>()
}

fn write_contents(contents: Vec<String>) {
    println!("{}", contents.join("\n"));
}
fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let contents = read_file(buffer.as_str());
    write_contents(contents);
}
