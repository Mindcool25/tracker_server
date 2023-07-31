use std::io;
use rand::Rng;

use requests::ConnectRequest;
use tokio::net::UdpSocket;
use serde::{Serialize, Deserialize};

mod requests;
mod response;

enum actions {
    Connect = 0,
    Announce,
    Scrape,
    Error,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Bind to socket
    let sock = UdpSocket::bind("0.0.0.0:6969").await?;
    let mut buf: [u8; 1024] = [0; 1024];
    loop {
        // Accept connection and figure out what request the client sent
        let (_len, addr) = sock.recv_from(&mut buf).await?;
        let request: requests::Request = requests::Request::from_bytes(buf);

        let mut resp: Vec<u8> = Vec::new();

        match request.action {
            0 => resp = handle_connect(request).await, // Connect
            1 => resp = handle_announce(request).await,
            _ => println!("ah we messed up bad, action was {}", request.action),
        }

        sock.send_to(&resp, addr).await?;
    }
}

async fn handle_connect(request: requests::Request) -> Vec<u8> {
    let conn: requests::ConnectRequest = request.to_connet_request();
    let mut rng = rand::thread_rng();
    println!("{:?}", conn);

    let resp: response::ConnectResponse = response::ConnectResponse {
        transaction_id: conn.transaction_id,
        connection_id: rng.gen(),
    };
    resp.to_bytes()
}

async fn handle_announce(request: requests::Request) -> Vec<u8> {
    let ann: requests::AnnounceRequest = request.to_announce_request();
    let test_peer: response::Peer = response::Peer{ip_address: 2130706433, port: 14896};
    let mut peers: Vec<response::Peer> = Vec::new();
    peers.push(test_peer);
    let resp: response::AnnounceResponse = response::AnnounceResponse {
        transaction_id: ann.transaction_id,
        interval: 2,
        leechers: 1,
        seeders: 1,
        peers,

    };
    println!("announce response: {:?}", resp);
    resp.to_bytes()
}
