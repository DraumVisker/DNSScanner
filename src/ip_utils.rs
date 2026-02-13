use ipnetwork::Ipv4Network;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::Ipv4Addr;

pub fn load_targets(input: &str) -> Vec<Ipv4Addr> {
    // If file
    if input.ends_with(".txt") {
        let file = File::open(input).expect("Failed to open file");
        return BufReader::new(file)
            .lines()
            .filter_map(|l| l.ok()?.parse().ok())
            .collect();
    }

    // If CIDR
    if input.contains('/') {
        let network: Ipv4Network = input.parse().expect("Invalid CIDR");
        return network.iter().collect();
    }

    // Single IP
    vec![input.parse().expect("Invalid IP")]
}
