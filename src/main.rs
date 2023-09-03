use serde_derive::{Deserialize, Serialize};
use serde_json::{self, to_string_pretty};
use std::{
    env,
    fs::File,
    io::{self, BufReader, Read, Write},
    process::{Command, exit, Stdio},
};

use lazy_static::lazy_static;

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

lazy_static! {
    static ref FILE_PATH: String = {
        let home_dir = env::var("HOME").expect("HOME environment variable not set");
        format!("{}/.config/sshelper/connections.json", home_dir)
    };
}

fn main() {
    let mut data = read_and_parse_json();
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("add") => add_connection(&mut data, &*FILE_PATH),
        Some("list") => list_connection_names(&data),
        Some("connect") => {
            if let Some(name) = args.get(2) {
                connect_to_ssh(&data, name);
            } else {
                println!("Please provide a connection name");
            }
        }
        _ => println!("Invalid command. Use 'add', 'list' or 'connect'"),
    }
}

fn read_and_parse_json() -> Data {
    let file = File::open(&*FILE_PATH).expect("File not found");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Unable to read file");
    serde_json::from_str(&contents).unwrap()
}

fn add_connection(data: &mut Data, file_path: &str) {
    let mut name = String::new();
    let mut host = String::new();
    let mut port = String::new();
    let mut username = String::new();
    let mut password = String::new();
    let mut key_path = String::new();

    println!("Enter connection details:");
    print!("Name: ");
    io::stdout().flush().unwrap();
    read_input(&mut name, "Name");
    print!("Host: ");
    io::stdout().flush().unwrap();
    read_input(&mut host, "Host");
    print!("Port: ");
    io::stdout().flush().unwrap();
    read_input(&mut port, "Port");
    let port: u32 = port.trim().parse().expect("Failed to parse Port as u32");
    print!("Username: ");
    io::stdout().flush().unwrap();
    read_input(&mut username, "Username");
    print!("Password: ");
    io::stdout().flush().unwrap();
    read_input(&mut password, "Password");
    print!("Key Path: ");
    io::stdout().flush().unwrap();
    read_input(&mut key_path, "Key Path");
    let new_connection = Connection {
        name: name.trim().to_string(),
        host: host.trim().to_string(),
        port: port,
        username: username.trim().to_string(),
        password: Some(password.trim().to_string()),
        key_path: Some(key_path.trim().to_string()),
    };
    data.connections.push(new_connection);
    save_to_json(data, file_path);
}

fn save_to_json(data: &Data, file_path: &str) {
    let json = to_string_pretty(data).expect("Failed to serialize data");
    let mut file = File::create(file_path).expect("Failed to create file");
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");
}

fn read_input(input: &mut String, field_name: &str) {
    match io::stdin().read_line(input) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to read line for {}: {}", field_name, e);
            return;
        }
    }
}

fn list_connection_names(data: &Data) {
    for connection in &data.connections {
        println!("Name:{}", connection.name);
        println!("Host:{}", connection.host);
    }
}

fn connect_to_ssh(data: &Data, name: &str) {
    if let Some(connection) = data.connections.iter().find(|c| c.name == name) {
        println!("Trying to connect to {}", connection.name);
        let ssh_command = format!(
            "ssh -v {}@{} -p {}",
            connection.username, connection.host, connection.port
        );
        println!("SSH Command: {}", ssh_command);
        println!("Username: {}", connection.username);
        println!("Host: {}", connection.host);
        println!("Port: {}", connection.port);
        if let Some(password) = &connection.password {
            println!("Password: {}", password);
        }
        if let Some(key_path) = &connection.key_path {
            println!("Key Path: {}", key_path);
        }
        let child = Command::new("sh")
            .arg("-c")
            .arg(&ssh_command)
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Failed to execute command");
        let output = child
            .wait_with_output()
            .expect("Failed to wait on child");
        exit(output.status.code().unwrap_or(100));
    } else {
        println!("No connection found with the name: {}", name);
    }
}

