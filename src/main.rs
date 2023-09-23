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
        println!("Conn from {}", addr);
        let mut request: requests::Request = requests::Request::from_bytes(buf);
        // Get IP, set it to a u32, if IPv6, tell them that it isn't supported.
        let conn_ip = match addr.ip() {
            std::net::IpAddr::V4(ip4) => u32::from_be_bytes(ip4.octets()),
            _ => 0,
        };
        if conn_ip == 0 {
            request.action = 3;
        }
        let resp = match request.action {
            0 => handle_connect(request).await,           // Connect
            1 => handle_announce(request, conn_ip).await, // Announce
            2 => handle_scrape(request).await,            // Scrape
            3 => handle_error(request, "Invalid IP".to_string()).await,
            _ => handle_error(request, "Invalid action".to_string()).await,
        };
        sock.send_to(&resp, addr).await?;
    }
}

async fn handle_connect(request: requests::Request) -> Vec<u8> {
    let conn: requests::ConnectRequest = request.to_connect_request();
    let mut rng = rand::thread_rng();
    let resp: response::ConnectResponse = response::ConnectResponse {
        transaction_id: conn.transaction_id,
        connection_id: rng.gen(),
    };
    println!("{:?}", resp);
    resp.to_bytes()
}

async fn handle_announce(request: requests::Request, ip: u32) -> Vec<u8> {
    // Check if hash exists in database
    // If not, create new set, add peer to set
    // If yes, check set, get max seeder peers, then max leecher peers until number
    // of peers wanted is reached or peers run out
    // Return whatever is left
    let mut req: requests::AnnounceRequest = request.to_announce_request();
    println!("{:?}", req);
    if req.ip_address == 0 {
        req.ip_address = ip;
    }
    db::create_torrent_hash(&req).await;
    //db::get_count(req).await;
    let s_count: i32 = i32::try_from(db::get_seeder_count(&req).await).expect("oh no");
    let l_count: i32 = i32::try_from(db::get_leecher_count(&req).await).expect("oh no");
    let resp = response::AnnounceResponse {
        transaction_id: req.transaction_id,
        interval: 5,
        leechers: l_count,
        seeders: s_count,
        peers: db::get_peers(&req).await,
    };
    println!("{:?}", resp);
    resp.to_bytes()
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
