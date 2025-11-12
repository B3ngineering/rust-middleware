use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::time::Duration;
use std::env;
use bincode;
use messages::Message;


fn main() {
    let args: Vec<String> = env::args().collect();

    let message_type = Message::from_str(&args[1]).unwrap();
    let topic = args.get(2).map(|s| s.as_str()).unwrap_or(message_type.default_topic());
    let port = args.get(3)
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(9100);

    let addr = format!("127.0.0.1:{}", port);

    println!("Subscribing to {} messages on topic: {}", args[1], topic);

    // Register with master
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
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Subscriber listening on {}", addr);

    // For incoming messages, read to buffer and display
    for incoming in listener.incoming() {
        let mut stream = incoming.unwrap();
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).unwrap();
        match message_type {
            Message::Twist => {
                let twist: messages::Twist = bincode::deserialize(&buf[..n]).unwrap();
                println!("[SUB] Received {:?}: {:?}", message_type, twist);
            }
            Message::Odom => {
                let odom: messages::Odom = bincode::deserialize(&buf[..n]).unwrap();
                println!("[SUB] Received {:?}: {:?}", message_type, odom);
            }
        }
    }
}
