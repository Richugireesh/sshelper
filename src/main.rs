use std::fs::File;
use std::io::prelude::*;
fn main() {
    println!("Hello, world!");
    let json = read_json();
    print!("{}", json);
}

// function to read from a json file
fn read_json() -> String {
    let mut file = File::open("data.json").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

