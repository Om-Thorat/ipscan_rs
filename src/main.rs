use std::{thread::{self, JoinHandle}, time::{self, Duration}, net::{IpAddr, SocketAddr, ToSocketAddrs}};
use tokio::{self};
use pinger::{ping,PingResult};


fn main() {
    tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
    for j in 0..10 {
        let x = thread::spawn( move || init_scan(j));
        x.join().expect("shush").await;
    };
})
}

async fn init_scan(i:i32){
    let _socket_addresses: Vec<SocketAddr> = format!("172.27.72.73:0").to_socket_addrs().unwrap().collect();
    scan_port(format!("1.1.1.{i}").to_string()).await;
}

async fn scan_port(target: String) {
    let u : Option<String> = Default::default();
    let stream = ping(target,u).expect("Error pinging");
    for message in stream {
        match message {
            PingResult::Pong(duration, _) => {println!("{:?}", duration);break;},
            PingResult::Timeout(_) => {println!("Timeout!");break;},
            // Unknown lines, just ignore.
            PingResult::Unknown(_line) => (),
            PingResult::PingExited(_, _) => println!("ping exited"),
        }
    }
}
