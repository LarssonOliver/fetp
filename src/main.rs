mod auth;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    let string = "220 Service ready\r\n";
    stream.write(string.as_bytes()).unwrap();
    let mut buffer = String::new();
    let size = stream.read_to_string(&mut buffer).unwrap();
    println!("{} {:#?}", size, buffer);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8000")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
