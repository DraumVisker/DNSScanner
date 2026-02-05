mod scanner;
mod ip_utils;

use clap::Parser;
use std::net::IpAddr;

#[derive(Parser)]
struct Args {
    target: String,

    #[arg(short, long, default_value = "live_dns.txt")]
    output: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let ips: Vec<IpAddr> = ip_utils::parse_target(&args.target);

    println!("Loaded {} IPs", ips.len());

    scanner::scan_all(ips, &args.output).await;
}
