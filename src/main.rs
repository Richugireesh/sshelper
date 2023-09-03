use std::fs::File;
use std::io::Read;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Connection {
    name: String,
    host: String,
    port: u32,
    username: String,
    password: Option<String>,
    key_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    connections: Vec<Connection>,
}

fn main() {
    println!("Hello, world!");
    let data = read_and_parse_json("data/connections.json");
    println!("{:?}", data);
}

// Function to read and parse from a JSON file
fn read_and_parse_json(file_path: &str) -> Data {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let parsed_data: Data = serde_json::from_str(&contents).unwrap();
    parsed_data
}
