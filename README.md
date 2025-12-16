# Rust Port Scanner

A fast and asynchronous port scanner written in **Rust**, using the **Tokio** async runtime. 
This tool scans a target host to detect open TCP ports, identify common services and perform **banner grabbing**, including **HTTP header detection**.

## Authors

1. Nabeel Ahmad KAMALUDEEN -CCC1
2. John DIB - CCC1
3. Yassine SMACH - DIA

## Project Objective

The objective of this project is to understand:
- Computer networking fundamentals (IP addresses, ports, services)
- Asynchronous programming in Rust
- How port scanners work internally
- Safe and responsible security tool development

## Use Cases

- Quickly identify open ports on a local machine
- Discover running services on a network device
- Perform basic reconnaissance in a controlled lab environment
- Learn how real-world port scanners operate
- Educational demonstration of networking and security concepts

## Features

- Asynchronous TCP port scanning using Tokio
- FAST scan (common ports)
- WEB scan (HTTP/HTTPS-related ports)
- FULL scan (ports 1–65535)
- Custom port range scanning
- Service detection using port-to-service mapping
- Banner grabbing for services
- HTTP header grabbing for web services
- Clean, formatted one-line banner output
- Colored and readable terminal output

## Project Status

- Core port scanning functionality implemented
- Multiple scan modes supported (FAST, WEB, FULL, RANGE)
- Banner grabbing and HTTP header detection completed
- Code tested on local and private networks
- Project considered functionally complete for academic submission

## Installation & Compilation

### Requirements
- Rust (stable)
- Cargo

### Build the project
```bash
cargo build

Run the project
cargo run -- --host <IP_ADDRESS> --fast

Usage Examples:
FAST scan (common ports)
cargo run -- --host 127.0.0.1 --fast

WEB scan (web-related ports)
cargo run -- --host 192.168.0.1 --web

FULL scan (all ports)
cargo run -- --host 127.0.0.1 --full

Custom range scan
cargo run -- --host 127.0.0.1 --range 1 1024
```

## Scan Modes Explained

### FAST Scan
Scans commonly used ports such as SSH, HTTP, SMB, MySQL, etc.  
This mode is fast and suitable for quick reconnaissance.

### WEB Scan
Scans ports commonly used by web services such as HTTP and HTTPS.  
Useful for identifying web servers.

### FULL Scan
Scans all **65,535 TCP ports**.  
Provides a complete overview but takes more time.

### RANGE Scan
Scans a **user-defined port range** specified by the user.  
Allows focused scanning on specific ports.

## Banner Grabbing & HTTP Header Detection

- For non-HTTP services, the scanner attempts to read service banners when available
- For HTTP ports, a GET request is sent and HTTP headers are extracted
- Banners are cleaned and displayed as a single readable line
- This provides additional information about the running service and its software.

## Code Structure

### main.rs
Handles CLI parsing, async scanning logic, banner grabbing, and HTTP header detection

### services.rs
Contains the port-to-service mapping

## Libraries Used

- tokio – asynchronous networking
- clap – command-line argument parsing
- colored – colored terminal output

## Challenges Faced

- Managing asynchronous tasks efficiently  
  → Solved using Tokio async runtime and task spawning.

- Handling timeouts to avoid long scan times  
  → Implemented connection and read timeouts.

- Cleaning binary banner data into readable text  
  → Applied banner sanitization and one-line formatting.

- Understanding why some services do not return banners  
  → Learned that many services wait for client input or block banners.

- Firewall and network restrictions  
  → Tested on local networks and loopback addresses.


## Limitations & Future Improvements

- HTTPS banner grabbing is limited due to TLS encryption
- No operating system detection
- No UDP scanning
- Export scan results to JSON or CSV
- Add concurrency limits and performance tuning

## Responsible Usage Warning 

This tool is developed for educational purposes only.

## Only scan:

- Our own devices
- Networks where you have explicit permission
- Unauthorized scanning of systems is illegal and unethical.

## Conclusion

This project demonstrates a practical understanding of networking,
asynchronous programming, and security concepts using Rust.
By building a functional port scanner with banner grabbing,
we gained insight into how real-world security tools operate.
The project also emphasizes ethical and responsible usage of such tools.

