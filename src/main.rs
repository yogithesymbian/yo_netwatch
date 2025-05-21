use crossbeam_channel::{Receiver, unbounded};
use eframe::egui;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Network Status App",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    )
}

struct MyApp {
    connected: bool,
    rx: Receiver<bool>,
}

impl MyApp {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();

        // Background thread to detect connectivity change
        thread::spawn(move || {
            let mut last_state = false;

            loop {
                let current_state = TcpStream::connect("8.8.8.8:53").is_ok();
                if current_state != last_state {
                    tx.send(current_state).ok(); // Notify UI
                    last_state = current_state;
                }

                // Check every second — light operation
                thread::sleep(Duration::from_secs(1));
            }
        });

        Self {
            connected: false,
            rx,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for updates from background thread
        while let Ok(status) = self.rx.try_recv() {
            self.connected = status;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📡 Internet Connection Status:");
            ui.add_space(10.0);
            ui.label(if self.connected {
                "✅ Online"
            } else {
                "❌ Offline"
            });
        });

        ctx.request_repaint_after(Duration::from_millis(100)); // keeps UI responsive
    }
}
