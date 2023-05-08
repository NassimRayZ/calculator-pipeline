const TRACE_SOCKET_PATH: &str = "/tmp/trace_calc.sock";
use std::{fs, os::unix::net::UnixStream};
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
                handle_request(&buf, len).await;
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {:#?}", e);
                continue;
            }
        }
    }
    let stdin = std::io::stdin();
    for line in stdin.lines() {
        println!("{}", line.unwrap());
    }
}

async fn handle_request(buf: &[u8], len: usize) {
    bu
}
