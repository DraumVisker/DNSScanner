use clap::Parser;
use std::fs;
use std::net::Ipv4Addr;
use std::str::FromStr;

mod scanner;
mod ip_utils;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    target: String,

    #[arg(short, long, default_value_t = 1000)]
    concurrency: usize,

    #[arg(short, long, default_value = "8.8.8.8")]
    resolver: String,

    #[arg(short, long, default_value = "live_dns.txt")]
    output: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut targets: Vec<Ipv4Addr> = Vec::new();

    if std::path::Path::new(&args.target).exists() {
        let content = fs::read_to_string(&args.target).unwrap();
        for line in content.lines() {
            if let Ok(ip) = Ipv4Addr::from_str(line.trim()) {
                targets.push(ip);
            }
        }
    } else if let Ok(ip) = Ipv4Addr::from_str(&args.target) {
        targets.push(ip);
    } else {
        targets = ip_utils::expand_cidr(&args.target);
    }

    scanner::run_scan(
        targets,
        vec![args.resolver],
        args.concurrency,
        args.output,
    )
        .await;
}
