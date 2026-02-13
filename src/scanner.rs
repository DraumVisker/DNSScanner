use std::fs::File;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use tokio::net::UdpSocket;
use tokio::sync::{Semaphore, mpsc};
use tokio::time::{timeout, Duration};

pub async fn run_scan(
    targets: Vec<Ipv4Addr>,
    resolver: String,
    concurrency: usize,
    output_file: String,
) {
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let live_counter = Arc::new(AtomicUsize::new(0));

    let (tx, mut rx) = mpsc::channel::<Ipv4Addr>(10000);

    // Writer Task
    let counter_clone = live_counter.clone();
    tokio::spawn(async move {
        let mut file = File::create(output_file).expect("Cannot create output file");

        while let Some(ip) = rx.recv().await {
            println!("[LIVE] {}", ip);
            writeln!(file, "{}", ip).ok();
            counter_clone.fetch_add(1, Ordering::Relaxed);
        }
    });

    // Spawn scan tasks
    for ip in targets {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let tx_clone = tx.clone();
        let resolver_clone = resolver.clone();

        tokio::spawn(async move {
            let _p = permit;

            if query_dns(ip, &resolver_clone).await {
                tx_clone.send(ip).await.ok();
            }
        });
    }

    drop(tx);

    // Give tasks time to finish
    tokio::time::sleep(Duration::from_secs(3)).await;

    println!(
        "\nScan complete. Total LIVE DNS: {}",
        live_counter.load(Ordering::Relaxed)
    );
}

async fn query_dns(ip: Ipv4Addr, resolver: &str) -> bool {
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(_) => return false,
    };

    let resolver_addr: SocketAddr =
        format!("{}:53", resolver).parse().expect("Invalid resolver");

    let packet = build_dns_packet();

    if socket.send_to(&packet, resolver_addr).await.is_err() {
        return false;
    }

    let mut buf = [0u8; 512];

    match timeout(Duration::from_secs(2), socket.recv_from(&mut buf)).await {
        Ok(Ok(_)) => true,
        _ => false,
    }
}

fn build_dns_packet() -> Vec<u8> {
    vec![
        0x12, 0x34,
        0x01, 0x00,
        0x00, 0x01,
        0x00, 0x00,
        0x00, 0x00,
        0x00, 0x00,
        0x03, b'w', b'w', b'w',
        0x06, b'g', b'o', b'o', b'g', b'l', b'e',
        0x03, b'c', b'o', b'm',
        0x00,
        0x00, 0x01,
        0x00, 0x01,
    ]
}
