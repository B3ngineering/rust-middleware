use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::time::Duration;
use bincode;
use messages::{Twist, Odom};


fn main() {
    println!("Hello, world!");

    let topic = "/odom";
    let addr = "127.0.0.1:9100";

    // Register with master
    println!("Attempting to register with master...");
    loop {
        match TcpStream::connect("127.0.0.1:9000") {
            Ok(mut stream) => {
                let msg = format!("SUB {} {}", topic, addr);
                match stream.write_all(msg.as_bytes()) {
                    Ok(_) => {
                        let _ = stream.flush();
                        println!("Successfully registered with master for topic {}", topic);
                        break;
                    }
                    Err(e) => {
                        println!("Failed to send registration: {}. Retrying in 2 seconds...", e);
                        std::thread::sleep(Duration::from_secs(2));
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect to master: {}. Retrying in 2 seconds...", e);
                std::thread::sleep(Duration::from_secs(2));
            }
        }
    }
    // Listen for messages
    let listener = TcpListener::bind(addr).unwrap();
    println!("Subscriber listening on {}", addr);

    // For incoming messages, read to buffer and display
    for incoming in listener.incoming() {
        let mut stream = incoming.unwrap();
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).unwrap();
        let msg: Odom = bincode::deserialize(&buf[..n]).unwrap();
        println!("[SUB] Received Twist: {:?}", msg);
    }
}
