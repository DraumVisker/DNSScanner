use std::fs::File;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use tokio::net::UdpSocket;
use tokio::sync::{Semaphore, Mutex};
use tokio::time::{timeout, Duration};

pub async fn run_scan(
    targets: Vec<Ipv4Addr>,
    resolvers: Vec<String>,
    concurrency: usize,
    output_file: String,
) {
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let file = Arc::new(Mutex::new(File::create(output_file).unwrap()));
    let live_counter = Arc::new(Mutex::new(0usize));

    let mut tasks = Vec::new();

    for ip in targets {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let file = file.clone();
        let counter = live_counter.clone();
        let resolvers = resolvers.clone();

        tasks.push(tokio::spawn(async move {
            let _permit = permit;

            if check_dns(ip, &resolvers).await {
                println!("[DNS] {}", ip);

                {
                    let mut f = file.lock().await;
                    writeln!(f, "{}", ip).ok();
                }

                let mut c = counter.lock().await;
                *c += 1;
            }
        }));
    }

    for t in tasks {
        let _ = t.await;
    }

    let total = *live_counter.lock().await;
    println!("\nTotal live DNS servers: {}", total);
}

async fn check_dns(ip: Ipv4Addr, _resolvers: &[String]) -> bool {
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(_) => return false,
    };

    let addr: SocketAddr = format!("{}:53", ip).parse().unwrap();
    let query = build_dns_query();

    for _ in 0..3 {
        let _ = socket.send_to(&query, addr).await;

        let mut buf = [0u8; 512];

        let result = timeout(Duration::from_secs(3), socket.recv_from(&mut buf)).await;

        if let Ok(Ok((size, _))) = result {
            if size > 12 {
                let flags = buf[2];
                let rcode = buf[3] & 0x0F;

                if flags & 0x80 == 0x80 && rcode == 0 {
                    return true;
                }
            }
        }
    }

    false
}

fn build_dns_query() -> Vec<u8> {
    let mut packet = vec![
        0x12, 0x34,
        0x01, 0x00,
        0x00, 0x01,
        0x00, 0x00,
        0x00, 0x00,
        0x00, 0x00,
    ];

    packet.extend(&[
        0x06, b'g', b'o', b'o', b'g', b'l', b'e',
        0x03, b'c', b'o', b'm',
        0x00,
    ]);

    packet.extend(&[
        0x00, 0x01,
        0x00, 0x01,
    ]);

    packet
}
