use colored::*;
use std::env;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

// Simple map of well-known services
fn get_service(port: u16) -> &'static str {
    match port {
        135 => "msrpc",
        139 => "netbios-ssn",
        445 => "smb",
        20 | 21 => "ftp",
        22 => "ssh",
        23 => "telnet",
        25 => "smtp",
        53 => "dns",
        80 => "http",
        110 => "pop3",
        143 => "imap",
        443 => "https",
        3306 => "mysql",
        3389 => "rdp",
        _ => "",
    }
}

async fn scan_port(host: &str, port: u16) -> Option<u16> {
    let addr = format!("{}:{}", host, port);

    // 200 ms timeout (fast)
    let duration = Duration::from_millis(200);

    // Try to connect within timeout
    match timeout(duration, TcpStream::connect(&addr)).await {
        Ok(Ok(_stream)) => Some(port),
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("{}", "Usage: rust-port-scanner <host> <start_port> <end_port>".yellow());
        println!("Example: rust-port-scanner 127.0.0.1 1 1000");
        return;
    }

    let host = &args[1];
    let start_port: u16 = args[2].parse().expect("Invalid start port");
    let end_port: u16 = args[3].parse().expect("Invalid end port");

    println!(
        "{} {} {} {}",
        "[*] Scanning".cyan(),
        host.green().bold(),
        "from port".cyan(),
        format!("{} to {}", start_port, end_port).green().bold()
    );

    println!("------------------------------------------------------");
    println!(
        "{:<8} {:<10} {:<10}",
        "PORT", "STATE", "SERVICE"
    );
    println!("------------------------------------------------------");

    // Create tasks for async scanning
    let mut handles = vec![];

    for port in start_port..=end_port {
        let host_clone = host.clone();
        handles.push(tokio::spawn(async move {
            scan_port(&host_clone, port).await
        }));
    }

    // Collect results
    for handle in handles {
        if let Ok(result) = handle.await {
            if let Some(port) = result {
                let service = get_service(port);
                println!(
                    "{:<8} {:<10} {:<10}",
                    port.to_string().green(),
                    "OPEN".green().bold(),
                    service
                );
            }
        }
    }

    println!("------------------------------------------------------");
    println!("{}", "[*] Scan complete.".cyan());
}
