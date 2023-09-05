// Importing necessary libraries and modules
use serde_derive::{Deserialize, Serialize}; // For JSON serialization and deserialization
use serde_json::{self, to_string_pretty}; // For handling JSON data
use std::{
    env, // For accessing environment variables
    fs::File, // For handling file operations
    io::{BufReader, Write, stdout, stdin}, // For input/output operations
    process::{Command, exit, Stdio}, // For handling system processes
};

// For handling static variables
use lazy_static::lazy_static;

// Struct to hold connection details
#[derive(Debug, Deserialize, Serialize)]
struct Connection {
    name: String, // Name of the connection
    host: String, // Host of the connection
    port: u32, // Port number for the connection
    username: String, // Username for the connection
    key_path: Option<String>, // Path to the key file (if any)
}

// Struct to hold all connections
#[derive(Debug, Deserialize, Serialize)]
struct Data {
    connections: Vec<Connection>, // Vector to hold all connections
}

// Static variable to hold the path to the JSON file
lazy_static! {
    static ref FILE_PATH: String = {
        let home_dir = env::var("HOME").expect("HOME environment variable not set"); // Get the home directory
        format!("{}/.config/sshelper/connections.json", home_dir) // Format the path to the JSON file
    };
}

// Main function
fn main() {
    let mut data = read_and_parse_json(); // Read and parse the JSON file
    let args: Vec<String> = env::args().collect(); // Collect command line arguments
    match args.get(1).map(String::as_str) { // Match the command
        Some("add") => add_connection(&mut data, &*FILE_PATH), // Add a new connection
        Some("list") => list_connection_names(&data), // List all connections
        Some("connect") => { // Connect to a connection
            if let Some(name) = args.get(2) {
                connect_to_ssh(&data, name); // Connect to the SSH
            } else {
                println!("Please provide a connection name"); // Error message
            }
        }
        _ => println!("Invalid command. Use 'add', 'list' or 'connect'"), // Default case
    }
}

// Function to read and parse the JSON file
fn read_and_parse_json() -> Data {
    let file = File::open(&*FILE_PATH).expect("File not found"); // Open the file
    let buf_reader = BufReader::new(file); // Create a buffer reader
    serde_json::from_reader(buf_reader).expect("Unable to read file") // Parse the JSON file
}

// Function to add a new connection
fn add_connection(data: &mut Data, file_path: &str) {
    let mut name = String::new(); // Name of the connection
    let mut host = String::new(); // Host of the connection
    let mut port = String::new(); // Port of the connection
    let mut username = String::new(); // Username of the connection
    let mut key_path = String::new(); // Path to the key file

    println!("Enter connection details:"); // Prompt for connection details
    print!("Name: ");
    stdout().flush().unwrap();
    read_input(&mut name, "Name"); // Read the name
    print!("Host: ");
    stdout().flush().unwrap();
    read_input(&mut host, "Host"); // Read the host
    print!("Port: ");
    stdout().flush().unwrap();
    read_input(&mut port, "Port"); // Read the port
    let port: u32 = port.trim().parse().expect("Failed to parse Port as u32"); // Parse the port as u32
    print!("Username: ");
    stdout().flush().unwrap();
    read_input(&mut username, "Username"); // Read the username
    print!("Absolute Key Path: ");
    stdout().flush().unwrap();
    read_input(&mut key_path, "Key Path"); // Read the key path
    let new_connection = Connection { // Create a new connection
        name: name.trim().to_string(),
        host: host.trim().to_string(),
        port: port,
        username: username.trim().to_string(),
        key_path: Some(key_path.trim().to_string()),
    };
    data.connections.push(new_connection); // Add the new connection to the list
    save_to_json(data, file_path); // Save the list to the JSON file
}

// Function to save the list to the JSON file
fn save_to_json(data: &Data, file_path: &str) {
    let json = to_string_pretty(data).expect("Failed to serialize data"); // Serialize the data
    let mut file = File::create(file_path).expect("Failed to create file"); // Create the file
    file.write_all(json.as_bytes()) // Write the data to the file
        .expect("Failed to write to file");
}

// Function to read input from the user
fn read_input(input: &mut String, field_name: &str) {
    match stdin().read_line(input) { // Read the input
        Ok(_) => {}
        Err(e) => {
            println!("Failed to read line for {}: {}", field_name, e); // Error message
            return;
        }
    }
}

// Function to list all connection names
fn list_connection_names(data: &Data) {
    for connection in &data.connections { // For each connection
        println!("Name:{}", connection.name); // Print the name
        println!("Host:{}", connection.host); // Print the host
    }
}
// Function to connect to a SSH
fn connect_to_ssh(data: &Data, name: &str) {
    if let Some(connection) = data.connections.iter().find(|c| c.name == name) { // If the connection exists
        println!("Trying to connect to {}", connection.name); // Print the connection name
        let ssh_command = if let Some(key_path) = &connection.key_path { // If the key path exists
            format!(
                "ssh -v -i {} {}@{} -p {}", // Format the SSH command
                key_path, connection.username, connection.host, connection.port
            )
        } else {
            format!(
                "ssh -v {}@{} -p {}", // Format the SSH command
                connection.username, connection.host, connection.port
            )
        };
        println!("SSH Command: {}", ssh_command); // Print the SSH command
        println!("Username: {}", connection.username); // Print the username
        println!("Host: {}", connection.host); // Print the host
        println!("Port: {}", connection.port); // Print the port
        if let Some(key_path) = &connection.key_path {
            println!("Key Path: {}", key_path); // Print the key path
        }
        let child = Command::new("sh") // Create a new shell command
            .arg("-c")
            .arg(&ssh_command) // Add the SSH command
            .stdout(Stdio::inherit()) // Inherit the standard output
            .spawn() // Spawn the command
            .expect("Failed to execute command"); // Error message
        let output = child
            .wait_with_output() // Wait for the command to finish
            .expect("Failed to wait on child"); // Error message
        exit(output.status.code().unwrap_or(100)); // Exit with the status code
    } else {
        println!("No connection found with the name: {}", name); // Error message
    }
}

