
# DNSScanner

High‑speed asynchronous DNS discovery tool written in Rust.
This scanner detects **live DNS servers** by sending UDP DNS queries concurrently and records responding servers to a file.

The project supports:

* Single IP scanning
* CIDR range scanning
* File‑based IP list scanning
* Async high‑speed probing using Tokio
* Automatic output of live DNS servers
* Live DNS counter display

---

## Features

* Async UDP DNS probing
* Thousands of concurrent scans
* CIDR / file / single‑IP input support
* Output results saved to file
* Real‑time console output
* Total live DNS counter at completion

---

## Project Structure

```
DNSScanner/
 ├── Cargo.toml
 ├── README.md
 └── src/
     ├── main.rs
     ├── scanner.rs
     └── ip_utils.rs
```

---

## Requirements

* Rust 1.70+
* Cargo

Install Rust if needed:

```bash
curl https://sh.rustup.rs -sSf | sh
```

---

## Build

Release build recommended for performance:

```bash
cargo build --release
```

Binary location:

```
target/release/DNSScanner
```

---

## Usage

### Scan a single IP

```bash
cargo run --release -- 8.8.8.8
```

---

### Scan a CIDR range

```bash
cargo run --release -- 8.8.8.0/24
```

---

### Scan from file

Create a file:

```
ips.txt
1.1.1.1
8.8.8.8
9.9.9.9
```

Run:

```bash
cargo run --release -- ips.txt
```

---

## Output

Example console output:

```
ACTIVE [UDP] 8.8.8.8
ACTIVE [UDP] 1.1.1.1
ACTIVE [UDP] 9.9.9.9

Total live DNS found: 3
```

Detected DNS servers are written to:

```
live_dns.txt
```

---

## How it works

The scanner:

1. Expands input targets (CIDR / file / IP)
2. Sends asynchronous UDP DNS queries to port 53
3. Waits for DNS responses
4. Logs responding servers
5. Writes results to output file
6. Displays total number of live DNS servers

This method allows extremely fast DNS discovery compared to sequential scanning.

---

## Performance Tips

* Always use release mode:

```bash
cargo run --release -- <target>
```

* Scan large networks from a VPS or high‑bandwidth environment
* Avoid scanning extremely large ranges without rate limiting

---

## Educational Purpose

This project is intended for:

* Network research
* DNS infrastructure discovery
* Learning async Rust networking
* Security laboratory environments

Ensure you have authorization before scanning networks you do not own.

---

## License

GPL-3.0 license

---

## Future Improvements

Planned enhancements:

* Recursive resolver detection
* Rate limiting / adaptive concurrency
* JSON output support
* DNS type detection (authoritative vs resolver)
* Mass‑scale scanning mode

