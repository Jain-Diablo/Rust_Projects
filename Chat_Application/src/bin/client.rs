use std::io::{self, BufRead, Write};
use std::net::TcpStream;
use std::thread;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let mut stdin = io::stdin().lock();

    // Spawn a thread to listen for messages from the server
    let stream_clone = stream.try_clone().unwrap();
    thread::spawn(move || {
        let mut buffer = String::new();
        let mut reader = io::BufReader::new(stream_clone);

        while let Ok(_) = reader.read_line(&mut buffer) {
            if !buffer.is_empty() {
                print!("{}", buffer); // Display server messages
                buffer.clear();
            }
        }
    });

    // Send messages to the server or quit if user types "exit"
    let mut input = String::new();
    while stdin.read_line(&mut input).is_ok() {
        let trimmed_input = input.trim();
        
        if trimmed_input == "exit" || trimmed_input == "quit" {
            println!("Exiting...");
            stream.write_all(b"Goodbye!\n").unwrap();
            break; // Exit the loop and disconnect from the server
        }

        if !trimmed_input.is_empty() {
            stream.write_all(input.as_bytes()).unwrap();
            stream.flush().unwrap();
        }

        input.clear();
    }

    // Close the stream before exiting
    drop(stream); // This will close the connection when dropped
}
