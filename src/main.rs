use std::{thread::{self}, vec};
use pinger::{ping,PingResult};
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    ip: String,
}

fn main() {
    let args = Cli::parse();
    let mut handles = vec![];
    let mut alive = 0;
    for j in 0..255 {
        let a = args.ip.clone();
        let handle = thread::spawn( move || init_scan(format!("{a}.{j}").to_string()));
        handles.push(handle);
    };
    for i in handles{
        alive += i.join().unwrap();
    }
    println!("Total {} alive out of 255!",alive);
}

fn init_scan(i:String) -> i32{
    if scan_ip(i){
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
