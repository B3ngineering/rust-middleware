use std::io::{Read, Write};
use std::net::TcpStream;


fn main() {
    println!("Hello, world!");

    let topic = "/cmd_vel";

    loop {
        // Check for subscribers from master
        let mut master = TcpStream::connect("127.0.0.1:9000").unwrap();
        let msg = format!("PUB {}", topic);
        master.write_all(msg.as_bytes()).unwrap();
        master.flush().unwrap();

        let mut buf = [0u8; 1024];
        let n = master.read(&mut buf).unwrap();
        let subs = String::from_utf8_lossy(&buf[..n]).to_string();

        for sub_addr in subs.split(',').filter(|s| !s.is_empty()) {
            let mut s = TcpStream::connect(sub_addr).unwrap();
            s.write_all("MOVE X Y".as_bytes()).unwrap();
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
