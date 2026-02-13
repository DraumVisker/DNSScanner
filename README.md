# DNSScanner

High-performance asynchronous DNS discovery tool written in Rust.
DNSScanner detects **live DNS servers** by sending concurrent UDP DNS queries and recording responding servers.

This tool is designed for **large-scale DNS infrastructure discovery**, **network research**, and **security laboratory environments**.

---

## Features

* Asynchronous UDP DNS probing
* High-speed concurrent scanning (Tokio runtime)
* Single IP scanning
* CIDR range scanning
* File-based target scanning
* Real-time live DNS detection output
* Automatic result saving to file
* Final live DNS counter summary
* Lightweight and fast Rust implementation

---

## Project Structure

```
DNSScanner/
├── Cargo.toml
├── dnsForTest.txt
├── live_dns.txt
└── src/
    ├── main.rs
    ├── scanner.rs
    └── ip_utils.rs
```

---

## Requirements

* Rust 1.70+
* Cargo

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

---

## Build

For maximum performance use release mode:

```bash
cargo build --release
```

Binary location:

```
target/release/DNSScanner
```

---

## Usage

### Scan single IP

```bash
DNSScanner -t 8.8.8.8
```

---

### Scan CIDR range

```bash
DNSScanner -t 8.8.8.0/24
```

---

### Scan from file

Example file:

```
ips.txt
1.1.1.1
8.8.8.8
9.9.9.9
```

Run:

```bash
DNSScanner -t ips.txt
```

---

## Example Output

```
[DNS] 8.8.8.8
[DNS] 1.1.1.1
[DNS] 9.9.9.9

Total live DNS found: 3
```

Detected DNS servers are saved to:

```
live_dns.txt
```

---

## How It Works

1. Expands input targets (single IP, CIDR, or file list)
2. Sends asynchronous UDP DNS queries to port 53
3. Waits for responses concurrently
4. Identifies responding DNS servers
5. Logs active DNS servers to output file
6. Displays total number of live DNS servers

This asynchronous approach enables extremely fast scanning compared to sequential methods.

---

## Performance Tips

* Always run in release mode:

  ```bash
  DNSScanner -t  <target>
  ```
* Use VPS or high-bandwidth environments for large scans
* Avoid scanning extremely large ranges without authorization

---

## Educational Purpose

This project is intended for:

* DNS infrastructure discovery
* Network measurement research
* Async Rust networking practice
* Security laboratory testing

Ensure you have authorization before scanning networks you do not own.

---

## License

GPL-3.0

