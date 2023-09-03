# sshelper
## Overview

`sshelper` is a tool that helps manage SSH connections. It stores connection details in a structured format, making it easy to manage multiple connections.

## Connection Structure

Each connection is represented as a struct with the following fields:

- `name`: A unique identifier for the connection.
- `host`: The hostname or IP address of the target machine.
- `port`: The port to connect to on the target machine.
- `username`: The username to use for the connection.
- `password`: The password to use for the connection. This field is optional.
- `key_path`: The path to the SSH key to use for the connection. This field is optional.

## Data Structure

The `Data` struct contains a vector of `Connection` structs. This allows `sshelper` to manage multiple connections.
