mod ip_utils;
mod scanner;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Target (CIDR / IP / file.txt)
    #[arg(short, long)]
    target: String,

    /// DNS resolver
    #[arg(short, long, default_value = "8.8.8.8")]
    resolver: String,

    /// Concurrency
    #[arg(short, long, default_value_t = 2000)]
    concurrency: usize,

    /// Output file
    #[arg(short, long, default_value = "live_dns.txt")]
    output: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let targets = ip_utils::load_targets(&args.target);

    scanner::run_scan(
        targets,
        args.resolver,
        args.concurrency,
        args.output,
    )
        .await;
}
