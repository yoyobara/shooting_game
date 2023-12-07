#[path = "../common.rs"]
mod common;

use std::net::TcpListener;

struct PlayerObject {
    name: String,
    position: (i16, i16),
}

fn main() {
    let mut players: Vec<PlayerObject> = Vec::with_capacity(10);

    let listener = TcpListener::bind("0.0.0.0:5001").unwrap();

    for client_stream in listener.incoming() {
        let client_stream = client_stream.unwrap();
    }
}
