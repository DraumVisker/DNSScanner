use futures::stream::{FuturesUnordered, StreamExt};
use std::fs::OpenOptions;
use std::io::Write;
use std::net::{IpAddr, SocketAddr};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::net::UdpSocket;
use tokio::time::{timeout, Duration};

pub async fn scan_all(ips: Vec<IpAddr>, outfile: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(outfile)
        .unwrap();

    let counter = Arc::new(AtomicUsize::new(0));

    let mut tasks = FuturesUnordered::new();

    for ip in ips {
        let counter_clone = counter.clone();
        tasks.push(scan_ip(ip, counter_clone));
    }

    while let Some(result) = tasks.next().await {
        if let Some(ip) = result {
            println!("ACTIVE [UDP] {}", ip);
            writeln!(file, "{}", ip).unwrap();
        }
    }

    println!(
        "\nTotal live DNS found: {}",
        counter.load(Ordering::Relaxed)
    );
}

async fn scan_ip(ip: IpAddr, counter: Arc<AtomicUsize>) -> Option<IpAddr> {
    let addr = SocketAddr::new(ip, 53);

    let socket = UdpSocket::bind("0.0.0.0:0").await.ok()?;

    let dns_query: [u8; 32] = [
        0x12,0x34,0x01,0x00,0x00,0x01,0x00,0x00,
        0x00,0x00,0x00,0x00,0x06,0x67,0x6f,0x6f,
        0x67,0x6c,0x65,0x03,0x63,0x6f,0x6d,0x00,
        0x00,0x01,0x00,0x01,0x00,0x00,0x00,0x00
    ];

    let _ = socket.send_to(&dns_query, addr).await.ok()?;

    let mut buf = [0u8; 512];

    let res = timeout(Duration::from_secs(2), socket.recv_from(&mut buf)).await;

    if res.is_ok() {
        counter.fetch_add(1, Ordering::Relaxed);
        return Some(ip);
    }

    None
}
