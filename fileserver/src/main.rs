use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:49151")?;

    loop {
        println!("Waiting for incoming connection...");
        let stream = listener.accept()?;
        println!("Connection established: {}", stream.0.peer_addr()?);

        let mut stream = stream.0;

        let mut filename = String::new();
        stream.read_to_string(&mut filename)?;
        println!("Received filename: {}", filename);

        // Perform necessary operations with filename here

        // Send response to client
        let response = "File received\n";
        stream.write_all(response.as_bytes())?;
        println!("Response sent to {}", stream.peer_addr()?);
    }
    
}
