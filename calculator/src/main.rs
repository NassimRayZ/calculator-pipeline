mod calculator;
mod common;
mod interpreter;
mod lexer;
mod parser;
const GUI_SOCKET_PATH: &str = "/tmp/gui_calc.sock";
const TRACE_SOCKET_PATH: &str = "/tmp/trace_calc.sock";

use std::fs;
use std::str::{self, Utf8Error};

use calculator::interpret;
use tokio::{
    io::{self, AsyncWriteExt},
    net::{UnixListener, UnixStream},
};
#[tokio::main]
async fn main() {
    if fs::metadata(GUI_SOCKET_PATH).is_ok() {
        fs::remove_file(GUI_SOCKET_PATH).expect("Failed to remove old socket");
    }
    let socket = UnixListener::bind(GUI_SOCKET_PATH).expect("Failed to bind socket path");

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
    let truncated_buf = &buf[0..len];
    let result = interpret(&truncated_buf, len);
    let response = result.to_ne_bytes();
    let full_op = match full_operation(truncated_buf, &result) {
        Ok(op) => op,
        Err(err) => {
            eprintln!("Failed to parse str from_utf8: {:#?}", err);
            stream
                .write_u8(0)
                .await
                .expect("Server Failed to send data");
            return;
        }
    };
    match tokio::join!(handle_trace(&full_op), stream.write(&response)) {
        (Ok(_), Err(e)) => eprintln!("Server failed to send gui data: {:#?}", e),
        (Err(e), Ok(_)) => eprintln!("Server failed to send trace data: {:#?}", e),
        (Err(e_trace), Err(e_gui)) => {
            eprintln!("Server failed to send data: [{:#?}, {:#?}]", e_trace, e_gui)
        }
        (Ok(_), Ok(_)) => {}
    }
}

fn full_operation<'a>(buf: &'a [u8], result: &f64) -> Result<String, Utf8Error> {
    let operation = str::from_utf8(buf)?;
    Ok(format!("{} = {}", operation, result))
}

async fn handle_trace(data: &str) -> io::Result<()> {
    let stream = UnixStream::connect(TRACE_SOCKET_PATH).await?;
    stream.writable().await?;
    stream.try_write(&data.as_bytes())?;
    Ok(())
}
