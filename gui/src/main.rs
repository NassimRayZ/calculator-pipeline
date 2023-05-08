#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod error;
mod parser;

use eframe::egui;
use open;
use parser::parse;
use std::future::Future;
use tokio::{net::UnixStream, runtime::Runtime};

const GUI_SOCKET_PATH: &str = "/tmp/gui_calc.sock";
const TRACE_FILE_PATH: &str = "/tmp/trace.txt";

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(420.0, 140.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Box::<Calculator>::default()),
    )
}

fn async_wrapper<F>(future: F) -> F::Output
where
    F: Future,
{
    Runtime::new().unwrap().block_on(future)
}

struct Calculator {
    operation: String,
    result: String,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            operation: "".to_owned(),
            result: "".to_owned(),
        }
    }
}
impl Calculator {
    async fn read_buffer(&mut self, socket: UnixStream) {
        let mut buf = [0u8; 1024];
        socket.readable().await.expect("Failed: socket unreadable");
        let truncated_buf = match socket.try_read(&mut buf) {
            Ok(len) => buf[0..len]
                .try_into()
                .expect("Failed: parsing f64 from native endian slice"),
            Err(e) => {
                eprintln!("Failed to read data from Unix socket: {:#?}", e);
                return;
            }
        };
        self.result = f64::from_ne_bytes(truncated_buf).to_string()
    }
    async fn handle_calculator(
        &mut self,
        socket: &UnixStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        parse(&self.operation)?;
        socket.writable().await.expect("Failed: socket unwritable");
        match socket.try_write(self.operation.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("Failed to send data through Unix socket {:#?}", e).into());
            }
        };
        Ok(())
    }
}

impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("small Calculator for simple operations");
            ui.horizontal(|ui| {
                let op_label = ui.label("your operation: ");
                ui.add_sized(
                    [140.0, 20.0],
                    egui::TextEdit::singleline(&mut self.operation),
                )
                .labelled_by(op_label.id);
            });
            ui.horizontal(|ui| {
                if ui.button("submit").clicked() {
                    async_wrapper(async {
                        let socket = UnixStream::connect(GUI_SOCKET_PATH)
                            .await
                            .expect("Failed to connect to unix socket");
                        if let Err(e) = self.handle_calculator(&socket).await {
                            eprintln!("{}", e);
                        } else {
                            self.read_buffer(socket).await;
                        }
                    });
                }
                ui.label("          ");
                if ui.button("trace").clicked() {
                    open::that(TRACE_FILE_PATH).expect("Failed to open `Cargo.toml`");
                }
            });
            ui.label(format!(
                "Operation: '{}'\nResult: {}",
                self.operation, self.result
            ));
        });
    }
}
