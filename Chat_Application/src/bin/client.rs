use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::thread;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Could not connect to server");

    println!("Connected to the chat server! Type 'quit' to exit.");

    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");
    
    thread::spawn(move || {
        let mut buffer = [0; 512];
        loop {
            match stream_clone.read(&mut buffer) {
                Ok(0) => break, // Connection closed
                Ok(_) => {
                    let msg = String::from_utf8_lossy(&buffer);
                    print!("Received: {}", msg);
                }
                Err(_) => break,
            }
        }
    });

    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read input");
        if input.trim().eq_ignore_ascii_case("quit") {
            println!("Exiting...");
            break;
        }
        stream.write_all(input.as_bytes()).expect("Failed to send message");
    }
}
