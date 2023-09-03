use std::fs::File;
use std::io::Read;
use std::env;

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
    let home_dir = env::var("HOME").unwrap();
    let file_path = format!("{}/.config/sshelper/connections.json", home_dir);
    let data = read_and_parse_json(&file_path);
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
