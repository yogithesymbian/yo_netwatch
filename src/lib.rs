//! yo_netwatch: A Rust crate to watch network connectivity changes,
//! inspired by Flutter's connectivity_plus crate.
//!
//! Provides a background watcher thread that emits `true` (online)
//! or `false` (offline) through a channel whenever the network status changes.

pub use crossbeam_channel::Receiver;

use crossbeam_channel::unbounded;
use std::net::TcpStream;
use std::{thread, time::Duration};

pub fn start_network_watcher() -> Receiver<bool> {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        let mut last_status = None;

        loop {
            // Check connectivity by attempting to open a TCP connection to google DNS (8.8.8.8:53)
            let is_online =
                TcpStream::connect_timeout(&"8.8.8.8:53".parse().unwrap(), Duration::from_secs(2))
                    .is_ok();

            if last_status != Some(is_online) {
                // Send new status if it changed
                if tx.send(is_online).is_err() {
                    // Receiver dropped, exit thread
                    break;
                }
                last_status = Some(is_online);
            }

            // Sleep some time before next check (e.g., 5 seconds)
            thread::sleep(Duration::from_secs(5));
        }
    });

    rx
}
