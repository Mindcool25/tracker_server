use crate::response;
use redis::AsyncCommands;
use std::net::Ipv4Addr;

// Data I need:
//
// Store peers in a hash, with <hash>:<peer key> ip <ip addr> port <port> downloaded <downloaded> left <left> uploaded <uploaded>
// Set peer keys to expire after a few hours or so, so if they aren't changed they will automatically remove themselves.
// E.X. ``set key 100 ex 10``, key expires after 10 seconds.
// Maybe use a set for peers for a given hash? something like hash: [ip:port, ip:port, ...], then it can be quickly searched?
// Maybe optimize to give downloaders the best seeders? or at least the ones with them being done.... Maybe a seeders set and a leechers set?
// <hash>:seeders and <hash>:leechers, then just <hash> for all peers?

pub fn get_client() -> redis::Client {
    redis::Client::open("redis://127.0.0.1/").unwrap()
}

pub async fn set_json(r: &response::ConnectResponse) {
    // Connecting to redis
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_async_connection().await.unwrap();
    let _: () = conn.set("testkey", b"foo").await.unwrap();
    let _: () = redis::cmd("LPUSH")
        .arg(&["hash:peers", "IP:PORT"])
        .query_async(&mut conn)
        .await
        .unwrap();
    let test: Vec<String> = conn.smembers("settest").await.unwrap();
    println!("{:?}", test);
}

pub async fn check_hash(hash: [u8; 20]) -> bool {
    let client = get_client();
    let mut conn = client.get_async_connection().await.unwrap();
    let exists: i32 = redis::cmd("EXISTS")
        .arg(&[format!("{:?}:leechers", hash)])
        .query_async(&mut conn)
        .await
        .unwrap();
    exists == 1
}

pub async fn get_peers(hash: [u8; 20], amount: i32) -> response::Peers {
    // Setting up connection
    let client = get_client();
    let mut conn = client.get_async_connection().await.unwrap();

    // Set up return value
    let mut return_peers: response::Peers = response::Peers { peers: Vec::new() };

    // Setting amount to usize and such
    let peer_amount: usize;
    if amount == -1 {
        peer_amount = 80;
    } else {
        peer_amount = usize::try_from(amount).unwrap();
    }

    // Get seeders, if that isn't enough, get leechers as well.
    let mut db_peers: Vec<String> = conn.smembers(format!("{:?}:seeders", hash)).await.unwrap();
    if db_peers.len() < usize::try_from(peer_amount).unwrap() {
        db_peers = [
            db_peers,
            conn.smembers(format!("{:?}:leechers", hash)).await.unwrap(),
        ]
        .concat();
    }
    // Check if peer still exists here?
    // Add all peers to Peers struct and retrun
    // TODO: Maybe look at this and change to only using u32?
    for db_peer in db_peers {
        let address: Vec<&str> = db_peer.split(":").collect();
        let ip_address: Ipv4Addr = address[0].parse().unwrap();
        let port: u16 = address[1].parse().unwrap();
        return_peers.peers.push(response::Peer { ip_address, port })
    }

    // Trim down to correct size and return
    if return_peers.peers.len() > usize::try_from(amount).unwrap() {
        return_peers.peers.drain(peer_amount..);
    }
    return_peers
}

// TODO: Make a function that can convert u32 to ipv4 and vice versa
pub async fn add_peer(hash: [u8; 20], ip: u32, port: u16) {
    // Connecting to db
    let client = get_client();
    let mut conn = client.get_async_connection().await.unwrap();

    // Adding new set as a peer, then give it hash? Do I need to keep track of stuff at all or do I bs it?
    let _: () = conn.sadd(format!("{}:{}", ip, port), "").await.unwrap();
}
