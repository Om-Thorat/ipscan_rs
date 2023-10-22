use std::{thread::{self}, vec};
use pinger::{ping,PingResult};

fn main() {
    let mut handles = vec![];
    let mut alive = 0;
    for j in 0..255 {
        let handle = thread::spawn( move || init_scan(j));
        handles.push(handle);
    };
    for i in handles{
        alive += i.join().unwrap();
    }
    println!("Total {} alive out of 255!",alive);
}

fn init_scan(i:i32) -> i32{
    if scan_ip(format!("192.168.32.{i}").to_string()){
        return 1;
    };
    return 0;
}

fn scan_ip(target: String) -> bool{
    let mut alive = false;
    let interface : Option<String> = Default::default();
    let stream = ping(target.clone(),interface).expect("Error pinging");
    for message in stream {
        match message {
            PingResult::Pong(duration, _) => {println!("{:?} {}", duration, target);alive = true;break;},
            PingResult::Timeout(_) => {println!("Timeout! {target}");break;},
            // Unknown lines, just ignore.
            PingResult::Unknown(_line) => (),
            PingResult::PingExited(_, _) => println!("ping exited"),
        }
    }
    return alive;
}
