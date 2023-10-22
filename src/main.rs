use std::{thread::{self}, vec};
use pinger::{ping,PingResult};

fn main() {
    let mut u = vec![];
    let mut l = 0;
    for j in 0..255 {
        let x = thread::spawn( move || init_scan(j,&mut l));
        u.push(x);
    };
    for i in u{
        i.join().unwrap();
    }
    println!("{}",l);
}

fn init_scan(i:i32,l:&mut i32){
    if scan_ip(format!("192.168.32.{i}").to_string()){
        *l += 1;
    };
}

fn scan_ip(target: String) -> bool{
    let mut alive = false;
    let u : Option<String> = Default::default();
    let stream = ping(target.clone(),u).expect("Error pinging");
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
