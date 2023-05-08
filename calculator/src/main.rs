// #![allow(dead_code)]
// #![allow(unused)]
mod calculator;
mod common;
mod interpreter;
mod lexer;
mod parser;
const SOCKET_PATH: &str = "/tmp/gui_calc.sock";

use std::fs;

use calculator::interpret;
use tokio::{
    io::AsyncWriteExt,
    net::{UnixListener, UnixStream},
};
#[tokio::main]
async fn main() {
    if fs::metadata(SOCKET_PATH).is_ok() {
        fs::remove_file(SOCKET_PATH).expect("Failed to remove old socket");
    }
    let socket = UnixListener::bind(SOCKET_PATH).expect("Failed to bind socket path");

    loop {
        match socket.accept().await {
            Ok((stream, _)) => {
                let mut buf = [0u8; 1024];
                stream.readable().await.expect("Cannot read from socket");
                let len = match stream.try_read(&mut buf) {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("Failed to read from socket: {:#?}", e);
                        continue;
                    }
                };
                handle_request(stream, &buf, len).await;
            }
            Err(e) => {
                eprintln!("Failed to connect to client: {:#?}", e);
            }
        }
    }
}

async fn handle_request(mut stream: UnixStream, buf: &[u8], len: usize) {
    let result = interpret(buf, len).to_string();
    let response = result.as_bytes();
    stream
        .write(response)
        .await
        .expect("Server Failed to send data");
}
