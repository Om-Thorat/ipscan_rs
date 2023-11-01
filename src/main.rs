use std::{thread::{self}, vec};
use pinger::{ping,PingResult};
use clap::Parser;
use regex::Regex;
use local_ip_address::local_ip;
#[derive(Parser, Debug)]
struct Cli {
    ip: String,
}

fn main() {
    let mut args = Cli::parse();
    // Regex god truly
    let regip = Regex::new(r"^(([1-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\.){3}([1-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])$").unwrap();
    if args.ip == "a"{
            args.ip = local_ip().unwrap().to_string();
    } else if !regip.is_match(&args.ip){
        panic!("Not a valid ip")
    }
    let mut l:Vec<_> = args.ip.split('.').collect();
    l.pop();
    let ip = l.join(".");
    let mut handles = vec![];
    let mut alive = 0;
    for j in 0..255 {
        let a = ip.clone();
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
