use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream, topics: &mut HashMap<String, Vec<String>>) {
    // Input is a tcp string and map of topics
    let mut buf = [0u8; 1024];

    // Reads available bytes from the stream
    let n = stream.read(&mut buf).unwrap();
    let input = String::from_utf8_lossy(&buf[..n]);

    // This converts our input into a list of words 
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Parts will either be a subscriber, topic, address or publisher, topic
    // Based on which it is, we'll perform a specific action with match
    match parts.as_slice() {

        // Looks up / creates the topic in the map, inserts the addresses to the topic subscribers
        ["SUB", topic, addr] => {
            println!("Registering subscriber {} to topic {}", addr, topic);
            topics.entry(topic.to_string())
                .or_default()
                .push(addr.to_string());
            // Close connection after registration
            drop(stream);
        }

        // Adds subscriber addresses to response, and sends it back to the client
        ["PUB", topic] => {
            if let Some(subs) = topics.get(&topic.to_string()) {
                let response = subs.join(",");
                let _ = stream.write_all(response.as_bytes());
                let _ = stream.flush();
                println!("Sent {} subscribers for topic {}: {}", subs.len(), topic, response);
            } else {
                let _ = stream.write_all(b"");
                let _ = stream.flush();
                println!("No subscribers found for topic {}", topic);
            }
            // Explicitly close the connection by dropping the stream
            drop(stream);
        }
        _ => {}
    }
}

fn main() {
    println!("Hello, world!");

    // Creating a new TcpListener instance and a map for each topics
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    let mut topics: HashMap<String, Vec<String>> = HashMap::new();

    println!("Master running on 127.0.0.1:9000");

    // For each stream in our listener, handle it appropriately
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream, &mut topics);
    }
}
