use eframe::egui;
use yo_netwatch::{Receiver, start_network_watcher};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Network Status App",
        eframe::NativeOptions::default(),
        Box::new(|_| Box::new(MyApp::new())),
    )
}

struct MyApp {
    connected: bool,
    rx: Receiver<bool>,
}

impl MyApp {
    fn new() -> Self {
        let rx = start_network_watcher();
        Self {
            connected: false,
            rx,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        while let Ok(status) = self.rx.try_recv() {
            self.connected = status;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📡 Internet Status:");
            ui.label(if self.connected {
                "✅ Online"
            } else {
                "❌ Offline"
            });
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}
