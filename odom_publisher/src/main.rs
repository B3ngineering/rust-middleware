use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::Duration;
use messages::Odom;
use bincode;

fn main() {
    let topic = "/odom";

    loop {
        let mut master = TcpStream::connect("127.0.0.1:9000").unwrap();
        master.write_all(format!("PUB {}", topic).as_bytes()).unwrap();

        let mut buf = [0u8; 1024];
        let n = master.read(&mut buf).unwrap();
        let subs = String::from_utf8_lossy(&buf[..n]).to_string();

        for sub_addr in subs.split(',').filter(|s| !s.is_empty()) {
            let mut s = TcpStream::connect(sub_addr).unwrap();

            let msg = Odom {
                x: 1.0,
                y: 0.0,
                theta: 0.0,
                linear_velocity: 0.0,
                angular_velocity: 0.0,
            };

            let data = bincode::serialize(&msg).unwrap();
            s.write_all(&data).unwrap();
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}