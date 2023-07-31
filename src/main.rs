use std::io;
use rand::Rng;

use sql::Database;
use tokio::net::UdpSocket;

pub mod requests;
pub mod response;
mod sql;

enum _Actions {
    Connect = 0,
    Announce,
    Scrape,
    Error,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Set up database
    let db: Database = Database::new("test.db".to_string());
    // Bind to socket
    let sock = UdpSocket::bind("0.0.0.0:6969").await?;
    let mut buf: [u8; 1496] = [0; 1496];
    loop {
        // Accept connection and figure out what request the client sent
        let (_len, addr) = sock.recv_from(&mut buf).await?;
        let request: requests::Request = requests::Request::from_bytes(buf);

        let resp = match request.action {
            0 => handle_connect(request).await,      // Connect
            1 => db.handle_announce(request.to_announce_request(), &addr),     // Announce
            2 => handle_scrape(request).await,    // Scrape
            _ => handle_error(request, "Invalid action".to_string()).await,
        };
        println!("Response: {:?}", resp);
        sock.send_to(&resp, addr).await?;
    }
}

async fn handle_connect(request: requests::Request) -> Vec<u8> {
    let conn: requests::ConnectRequest = request.to_connet_request();
    let mut rng = rand::thread_rng();
    let resp: response::ConnectResponse = response::ConnectResponse {
        transaction_id: conn.transaction_id,
        connection_id: rng.gen(),
    };
    resp.to_bytes()
}

async fn handle_scrape(request: requests::Request) -> Vec<u8> {
    Vec::new()
}

async fn handle_error(request: requests::Request, message: String) -> Vec<u8> {
    response::ErrorResponse {
        transaction_id: i32::from_be_bytes(request.payload[0..4].try_into().unwrap()),
        message
    }.to_bytes()
}
