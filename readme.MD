# yo_netwatch

📡 A reactive internet connection status checker for `egui` apps — inspired by Flutter's `connectivity_plus`, built with ❤️ in Rust by Yo.

## Features

- Detects online/offline status via `TcpStream` ping
- Emits status only on **state changes** (not polling UI)
- Works with `egui` via `crossbeam-channel`
- Suitable for desktop apps with `eframe`

## Installation

Add to your `Cargo.toml`:

```toml
yo_netwatch = "0.1"
```

## Usage

```
use yo_netwatch::{start_network_watcher, Receiver};

fn main() {
    let rx: Receiver<bool> = start_network_watcher();

    loop {
        if let Ok(status) = rx.recv() {
            println!("Network is: {}", if status { "Online ✅" } else { "Offline ❌" });
        }
    }
}
```

## License
