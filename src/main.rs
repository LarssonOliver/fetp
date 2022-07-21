mod auth;
mod command;
mod config;
mod connection;
mod session;

use log::{debug, error, info};

use std::net::{TcpListener, TcpStream};

fn main() {
    init_logger();

    info!("Starting FeTP server...");

    listen(session::new);
}

fn init_logger() {
    let env = env_logger::Env::default().filter_or("FETP_LOG_LEVEL", "info");
    env_logger::init_from_env(env);
    debug!("Logger initialized");
}

fn listen(handler: fn(TcpStream)) {
    let listener = create_tcp_listener();

    info!("Ready to accept connections");

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

        handler(stream);
    }
}

fn create_tcp_listener() -> TcpListener {
    let listener = match TcpListener::bind(listen_address_formatted()) {
        Ok(listener) => listener,
        Err(e) => {
            error!(
                "Failed to bind to address {}: {}",
                listen_address_formatted(),
                e
            );
            std::process::exit(1);
        }
    };

    info!("Listening on {}", listener.local_addr().unwrap());
    return listener;
}

fn listen_address_formatted() -> String {
    format!("{}:{}", config::LISTEN_ADDR, config::LISTEN_PORT)
}
