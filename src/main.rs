mod auth;
mod command;
mod config;
mod connection;

use log::{debug, error, info};

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::connection::Connection;

fn handle_client(mut stream: TcpStream) {
    let string = "220 Service ready\r\n";
    stream.write(string.as_bytes()).unwrap();
    let mut buffer = String::new();
    let size = stream.read_to_string(&mut buffer).unwrap();
    info!("{} {:#?}", size, buffer);
}

fn main() {
    init_logger();

    info!("Starting FeTP server...");
    let listener = match TcpListener::bind(listen_address()) {
        Ok(listener) => listener,
        Err(e) => {
            error!("Failed to bind to address {}: {}", listen_address(), e);
            std::process::exit(1);
        }
    };

    info!("Listening on {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => {
                info!("Accepted connection from {}", stream.peer_addr().unwrap());
                stream
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
                continue;
            }
        };

        let mut connection = Connection::new(stream);

        connection
            .write_multiline(
                220,
                &[
                    "Welcome to FeTP!",
                    "",
                    "Please login using anonymous authentication.",
                ],
            )
            .expect("err");

        let buf = connection.read().unwrap();
        let cmd = command::parse(&buf).unwrap();
        info!("{:?} {:?}", cmd.verb, cmd.arg);
    }
}

fn init_logger() {
    let env = env_logger::Env::default().filter_or("FETP_LOG_LEVEL", "info");
    env_logger::init_from_env(env);
    debug!("Logger initialized");
}

fn listen_address() -> String {
    format!("{}:{}", config::LISTEN_ADDR, config::LISTEN_PORT)
}
