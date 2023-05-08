const TRACE_SOCKET_PATH: &str = "/tmp/trace_calc.sock";
const TRACE_FILE_PATH: &str = "/tmp/trace.txt";

use std::io::Write;
use std::{fs, str};
use tokio::net::UnixListener;

#[tokio::main]
async fn main() {
    if fs::metadata(TRACE_SOCKET_PATH).is_ok() {
        fs::remove_file(TRACE_SOCKET_PATH).expect("Failed to remove old socket");
    }
    let listener = UnixListener::bind(TRACE_SOCKET_PATH).expect("Failed: binding to unix socket");

    loop {
        match listener.accept().await {
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
                handle_request(&buf, len);
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {:#?}", e);
                continue;
            }
        }
    }
}

fn handle_request(buf: &[u8], len: usize) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(TRACE_FILE_PATH)
        .expect("Failed: opening the trace file");
    let operation = match str::from_utf8(&buf[0..len]) {
        Ok(op) => op,
        Err(e) => {
            eprintln!("Failed parsing the operation: {:#?}", e);
            return;
        }
    };
    if let Err(e) = writeln!(file, "{}", operation) {
        eprintln!("Failed writing to file: {:#?}", e);
    }
}
