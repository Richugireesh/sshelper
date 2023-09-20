# sshelper
## Overview

`sshelper` is a command-line tool that assists in managing SSH connections. It stores connection details in a structured JSON format, allowing for easy management of multiple connections.

## Connection Structure

Each connection is represented as a struct with the following fields:

- `name`: A unique identifier for the connection.
- `host`: The hostname or IP address of the target machine.
- `port`: The port to connect to on the target machine.
- `username`: The username to use for the connection.
- `password`: The password to use for the connection. This field is optional and it's recommended to use SSH keys instead for security reasons.
- `key_path`: The path to the SSH key to use for the connection. This field is optional.

## Data Structure

The `Data` struct contains a vector of `Connection` structs. This allows `sshelper` to manage multiple connections.

## Usage

`sshelper` supports the following commands:

- `add`: Add a new connection. You will be prompted to enter the connection details.
- `list`: List all saved connections by name and host.
- `connect <name>`: Connect to a saved connection by its name.

The connection details are stored in a JSON file located at `~/.config/sshelper/connections.json`.

## To-Do

- Implement a `delete` command to remove saved connections. ✅
- Add support for Windows.
- Improve error handling and user feedback.
- Add a `help` command to display usage information.
- Implement unit tests for all commands.
- Add support for encrypted password storage.
- Add support for password-based ssh connections.
- Add support for TUI(Maybe).



