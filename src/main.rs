mod services;
use services::load_services;

use clap::{Arg, ArgAction, Command};
use colored::*;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

// Clean the banner into a single readable line
fn clean_banner(banner: String) -> String {
    banner
        .replace('\n', " ")
        .replace('\r', " ")
        .replace('\t', " ")
        .chars()
        .filter(|c| !c.is_control()) // remove weird unprintable chars
        .collect::<String>()
        .trim()
        .to_string()
}

// Scan port + grab banner + HTTP header grab
async fn scan_port(host: &str, port: u16) -> Option<(u16, String)> {
    let addr = format!("{}:{}", host, port);
    let duration = Duration::from_millis(300);

    match timeout(duration, TcpStream::connect(&addr)).await {
        Ok(Ok(mut stream)) => {
            let mut banner = String::new();

            // If HTTP port, send GET request
            if port == 80 || port == 8080 || port == 8000 || port == 8888 || port == 8443 {
                let http_request = format!(
                    "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                    host
                );

                let _ = stream.write_all(http_request.as_bytes()).await;

                let mut buf = [0u8; 512];
                if let Ok(n) = stream.read(&mut buf).await {
                    if n > 0 {
                        let response = String::from_utf8_lossy(&buf[..n]).to_string();
                        if let Some(header) = response.split("\r\n\r\n").next() {
                            banner = header.to_string();
                        }
                    }
                }
            } else {
                // Generic banner grabbing
                let mut buf = [0u8; 256];
                if let Ok(Ok(n)) = timeout(Duration::from_millis(200), stream.read(&mut buf)).await
                {
                    if n > 0 {
                        banner = String::from_utf8_lossy(&buf[..n]).to_string();
                    }
                }
            }

            // return clean one-line banner
            Some((port, clean_banner(banner)))
        }
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let matches = Command::new("Rust Port Scanner")
        .version("1.3")
        .about("A fast async port scanner written in Rust with banner grabbing")
        .arg(
            Arg::new("host")
                .short('H')
                .long("host")
                .required(true)
                .help("Target host IP"),
        )
        .arg(
            Arg::new("fast")
                .long("fast")
                .help("Scan top common ports")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("web")
                .long("web")
                .help("Scan common web ports (80, 443, 8080, 8443)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("full")
                .long("full")
                .help("Scan all 65535 ports")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("range")
                .long("range")
                .num_args(2)
                .help("Manual port range: --range 1 1000"),
        )
        .get_matches();

    let host = matches.get_one::<String>("host").unwrap();

    // Decide scan type
    let ports: Vec<u16> = if matches.get_flag("fast") {
        vec![22, 53, 80, 135, 139, 443, 445, 3306, 3389]
    } else if matches.get_flag("web") {
        vec![80, 443, 8080, 8443]
    } else if matches.get_flag("full") {
        (1..=65535).collect()
    } else if let Some(range) = matches.get_many::<String>("range") {
        let r: Vec<u16> = range.map(|x| x.parse::<u16>().unwrap()).collect();
        (r[0]..=r[1]).collect()
    } else {
        println!(
            "{}",
            "Please provide --fast, --web, --full, or --range x y".yellow()
        );
        return;
    };

    println!("{} {}", "[*] Scanning host".cyan(), host.green().bold());

    println!(
        "------------------------------------------------------------------------------------------"
    );
    println!(
        "{:<8} {:<10} {:<15} {}",
        "PORT", "STATE", "SERVICE", "BANNER"
    );
    println!(
        "------------------------------------------------------------------------------------------"
    );

    let service_map = load_services();

    let mut handles = vec![];
    for port in ports.clone() {
        let h = host.clone();
        handles.push(tokio::spawn(async move { scan_port(&h, port).await }));
    }

    for handle in handles {
        if let Ok(Some((port, banner))) = handle.await {
            let service = service_map.get(&port).unwrap_or(&"");

            println!(
                "{:<8} {:<10} {:<15} {}",
                port.to_string().green(),
                "OPEN".green().bold(),
                service,
                banner
            );
        }
    }

    println!(
        "------------------------------------------------------------------------------------------"
    );
    println!("{}", "[*] Scan complete.".cyan());
}
