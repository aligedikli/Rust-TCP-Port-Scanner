# Rust-TCP-Port-Scanner

This is a simple, multithreaded TCP port scanner written in Rust. It's designed to scan a specified IP address for open ports within a defined range. This tool is useful for network administrators and security professionals to identify open ports on a networked device.

## Features

- **Fast Scanning**: Utilizes multithreading to scan multiple ports simultaneously, significantly speeding up the scanning process.
- **User-Friendly**: Prompts the user for an IP address to scan, making it easy to use without modifying the source code.
- **Versatile**: Allows scanning of all ports within the specified range (1-1024 by default), with the option to customize the range.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your machine. If you do not have Rust installed, you can follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository to your local machine:

```sh
git clone https://github.com/aligedikli/Rust-TCP-Port-Scanner.git
```

Change into the project directory:

```sh
cd rust-tcp-port-scanner
```

Build the project:

```sh
cargo build
```

Usage
To run the port scanner, use the following command:

```sh
cargo run
```

After running the command, you will be prompted to enter the IP address you wish to scan. The scanner will then proceed to check each port in the specified range and output a list of open ports.

## Contributing
Contributions are welcome! Please feel free to submit a pull request or open an issue for any bugs, improvements, or feature requests.

## Disclaimer
 This tool is for educational and network administration purposes only. Always ensure you have permission to scan the network or device. Unauthorized scanning can be illegal or against network policies.


 Remember to replace `https://github.com/aligedikli/Rust-TCP-Port-Scanner.git` with the actual URL of your GitHub repository. If you add or modify features, or if there are additional setup steps, make sure to update the README accordingly.

