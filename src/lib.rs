use crossbeam_channel::{Receiver, Sender, unbounded};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub fn start_network_watcher() -> Receiver<bool> {
    let (tx, rx) = unbounded();
    thread::spawn(move || network_watch_loop(tx));
    rx
}

fn network_watch_loop(tx: Sender<bool>) {
    let mut last_state = false;
    loop {
        let current = TcpStream::connect("8.8.8.8:53").is_ok();
        if current != last_state {
            let _ = tx.send(current);
            last_state = current;
        }
        thread::sleep(Duration::from_secs(1));
    }
}
