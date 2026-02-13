use std::net::Ipv4Addr;
use std::str::FromStr;

pub fn expand_cidr(cidr: &str) -> Vec<Ipv4Addr> {
    let mut result = Vec::new();

    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return result;
    }

    let base_ip = match Ipv4Addr::from_str(parts[0]) {
        Ok(ip) => ip,
        Err(_) => return result,
    };

    let prefix: u32 = match parts[1].parse() {
        Ok(p) => p,
        Err(_) => return result,
    };

    let host_bits = 32 - prefix;
    let total_ips = 1u32 << host_bits;

    let base: u32 = u32::from(base_ip);

    for i in 0..total_ips {
        result.push(Ipv4Addr::from(base + i));
    }

    result
}
