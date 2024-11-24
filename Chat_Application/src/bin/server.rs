use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection closed
            Ok(_) => {
                let msg = String::from_utf8_lossy(&buffer);
                println!("Client {} says: {}", stream.peer_addr().unwrap(), msg.trim());

                let clients = clients.lock().unwrap();
                for mut client in clients.iter() {
                    if client.peer_addr().unwrap() != stream.peer_addr().unwrap() {
                        client.write_all(msg.as_bytes()).unwrap();
                    }
                }
            }
            Err(_) => break,
        }
    }

    // Remove the client on disconnection
    let mut clients = clients.lock().unwrap();
    if let Some(index) = clients.iter().position(|s| s.peer_addr().unwrap() == stream.peer_addr().unwrap()) {
        clients.remove(index);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    println!("Server is running on 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                // Add the new client to the list
                {
                    let mut clients = clients.lock().unwrap();
                    clients.push(stream.try_clone().unwrap());
                }

                // Clone the Arc for the thread
                let clients_clone = Arc::clone(&clients);
                thread::spawn(move || handle_client(stream, clients_clone));
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}
