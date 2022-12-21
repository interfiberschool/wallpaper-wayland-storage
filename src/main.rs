use std::{os::unix::net::UnixListener, path::PathBuf, path::Path};
mod client;
mod manager;


pub fn get_socket_path() -> PathBuf {
    let runtime_dir = if let Ok(dir) = std::env::var("XDG_RUNTIME_DIR") {
        dir
    } else {
        "/tmp/wws".to_string()
    };
    let runtime_dir = Path::new(&runtime_dir);
    runtime_dir.join("wws.socket")
}

fn main() {
    let socket = UnixListener::bind(get_socket_path()).expect("Failed to bind to daemon socket");
    println!("Daemon bound to socket: {}", get_socket_path().to_str().unwrap());

    for client in socket.incoming() {
        let stream = client.expect("Client did not provide stream");

        // handle each client on a thread
        std::thread::Builder::new()
            .name(String::from("client_handler"))
            .spawn(move || {
                let mut c = client::Client::new(stream);
                c.handle();
            }).expect("Failed to spawn thread");
    }
}
