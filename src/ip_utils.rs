use ipnetwork::IpNetwork;
use std::fs;
use std::net::IpAddr;

pub fn parse_target(target: &str) -> Vec<IpAddr> {
    if target.contains('/') {
        let net: IpNetwork = target.parse().unwrap();
        return net.iter().collect();
    }

    if let Ok(content) = fs::read_to_string(target) {
        return content
            .lines()
            .filter_map(|l| l.parse().ok())
            .collect();
    }

    vec![target.parse().unwrap()]
}
