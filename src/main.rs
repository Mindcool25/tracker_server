use rand::Rng;
use std::io;

use tokio::net::UdpSocket;

pub mod db;
pub mod requests;
pub mod response;

enum _Actions {
    Connect = 0,
    Announce,
    Scrape,
    Error,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Bind to socket
    let sock = UdpSocket::bind("0.0.0.0:6969").await?;
    let mut buf: [u8; 1496] = [0; 1496];
    loop {
        // Accept connection and figure out what request the client sent
        let (_len, addr) = sock.recv_from(&mut buf).await?;
        let request: requests::Request = requests::Request::from_bytes(buf);

        let resp = match request.action {
            0 => handle_connect(request).await,  // Connect
            1 => handle_announce(request).await, // Announce
            2 => handle_scrape(request).await,   // Scrape
            _ => handle_error(request, "Invalid action".to_string()).await,
        };
        println!("Response: {:?}", resp);
        sock.send_to(&resp, addr).await?;
    }
}

async fn handle_connect(request: requests::Request) -> Vec<u8> {
    let conn: requests::ConnectRequest = request.to_connect_request();
    println!("{:?}", conn);
    let mut rng = rand::thread_rng();
    let resp: response::ConnectResponse = response::ConnectResponse {
        transaction_id: conn.transaction_id,
        connection_id: rng.gen(),
    };
    db::set_json(&resp).await;
    resp.to_bytes()
}

async fn handle_announce(request: requests::Request) -> Vec<u8> {
    // Check if hash exists in database
    // If not, create new set, add peer to set
    // If yes, check set, get max seeder peers, then max leecher peers until number
    // of peers wanted is reached or peers run out
    // Return whatever is left
    println!("{:?}", request.to_announce_request());
    Vec::new()
}

async fn handle_scrape(request: requests::Request) -> Vec<u8> {
    Vec::new()
}

async fn handle_error(request: requests::Request, message: String) -> Vec<u8> {
    response::ErrorResponse {
        transaction_id: i32::from_be_bytes(request.payload[0..4].try_into().unwrap()),
        message,
    }
    .to_bytes()
}
