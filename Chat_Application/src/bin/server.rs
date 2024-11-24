use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type ClientList = Arc<Mutex<Vec<(String, TcpStream)>>>;

fn handle_client(mut stream: TcpStream, clients: ClientList) {
    let mut buffer = [0; 512];

    // Ask for the client's username
    stream.write_all(b"Enter your username before you start to chat:\n").unwrap();
    stream.flush().unwrap(); // Ensure the prompt is sent immediately

    // Read the username
    let mut username = String::new();
    if let Ok(size) = stream.read(&mut buffer) {
        username = String::from_utf8_lossy(&buffer[..size]).trim().to_string();
    }
    if username.is_empty() {
        username = "Anonymous".to_string();
    }

    // Add the client to the shared list
    {
        let mut clients = clients.lock().unwrap();
        clients.push((username.clone(), stream.try_clone().unwrap()));
    }

    println!("{} has joined the chat!", username);

    // Notify all clients of the new connection
    broadcast_message(&clients, &format!("{} has joined the chat!", username), None);
    broadcast_message(&clients, &format!("It is our pleasure to have {} here", username), None);

    // Handle incoming messages from this client
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection closed
            Ok(size) => {
                let msg = String::from_utf8_lossy(&buffer[..size]).trim().to_string();
                if !msg.is_empty() {
                    let full_message = format!("{}: {}", username, msg);
                    println!("{}", full_message);
                    broadcast_message(&clients, &full_message, Some(&username));
                }
            }
            Err(_) => break,
        }
    }

    // Remove the client on disconnection
    {
        let mut clients = clients.lock().unwrap();
        if let Some(index) = clients.iter().position(|(user, _)| user == &username) {
            clients.remove(index);
        }
    }

    println!("{} has left the chat.", username);
    broadcast_message(&clients, &format!("{} has left the chat.", username), None);
}


fn broadcast_message(clients: &ClientList, message: &str, exclude: Option<&str>) {
    let mut clients = clients.lock().unwrap();
    for (user, client) in clients.iter_mut() {
        if Some(user.as_str()) != exclude {
            if let Err(e) = client.write_all(format!("{}\n", message).as_bytes()) {
                eprintln!("Failed to send message to {}: {}", user, e);
            }
        }
    }
}



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let clients: ClientList = Arc::new(Mutex::new(Vec::new()));

    println!("Server is running on 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                let clients_clone = Arc::clone(&clients);
                thread::spawn(move || handle_client(stream, clients_clone));
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}
