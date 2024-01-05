use std::{fs, io::{self, Read}};

fn read_file(file_name: &str) -> Vec<String> {
    String::from_utf8(fs::read(file_name).unwrap()).unwrap()
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
    let args = std::env::args().collect::<Vec<String>>();

    let contents = read_file(args[1].trim());
    write_contents(contents);
}
