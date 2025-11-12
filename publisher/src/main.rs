use std::io::{Read, Write};
use std::net::TcpStream;
use std::env;
use bincode;
use messages::{Message, Twist, Odom};

fn main() {
    let args: Vec<String> = env::args().collect();

    let message_type = Message::from_str(&args[1]).unwrap();
    let topic = args.get(2).map(|s| s.as_str()).unwrap_or_else(|| match message_type {
        Message::Twist => "/cmd_vel",
        Message::Odom => "/odom",
    });

    println!("Publishing {} messages on topic: {}", args[1], topic);

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

            let data = match message_type {
                Message::Twist => {
                    let twist = Twist {
                        linear_x: 1.0,
                        linear_y: 0.0,
                        linear_z: 0.0,
                        angular_x: 0.0,
                        angular_y: 0.0,
                        angular_z: 0.5,
                    };
                    bincode::serialize(&twist).unwrap()
                }
                Message::Odom => {
                    let odom = Odom {
                        x: 1.0,
                        y: 2.0,
                        theta: 0.5,
                        linear_velocity: 1.0,
                        angular_velocity: 0.5,
                    };
                    bincode::serialize(&odom).unwrap()
                }
            };
            s.write_all(&data).unwrap();
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
