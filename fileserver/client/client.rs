use std::io::{self, prelude::*};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:49151")?;

    let mut input = String::new();
    println!("Enter a filename:");
    io::stdin().read_line(&mut input)?;

    stream.write_all(input.as_bytes())?;
    println!("Filename sent to server.");

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("Response from server: {}", response);

}
