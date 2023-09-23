use crate::{
    requests::AnnounceRequest,
    response::{self, Peers},
};
use redis::AsyncCommands;

// Data I need:
//
// Store peers in a hash, with <hash>:<peer key> ip <ip addr> port <port> downloaded <downloaded> left <left> uploaded <uploaded>
// Set peer keys to expire after a few hours or so, so if they aren't changed they will automatically remove themselves.
// E.X. ``set key 100 ex 10``, key expires after 10 seconds.
// Maybe use a set for peers for a given hash? something like hash: [ip:port, ip:port, ...], then it can be quickly searched?
// Maybe optimize to give downloaders the best seeders? or at least the ones with them being done.... Maybe a seeders set and a leechers set?
// <hash>:seeders and <hash>:leechers, then just <hash> for all peers?
//
// Maybe closer to leechers:<hash> and seeders:<hash>, which both store things as <ip>:<port>?
// Then peer:<connection_id> which holds downloaded:<hash> <dl> left:<hash> <ldl> uploaded:<hash> <up>?

pub fn get_client() -> redis::Client {
    redis::Client::open("redis://127.0.0.1/").unwrap()
}

pub async fn check_hash(hash: [u8; 20]) -> bool {
    let client = get_client();
    let mut conn = client.get_async_connection().await.unwrap();
    let exists: i32 = redis::cmd("EXISTS")
        .arg(&[format!("leechers:{:?}", hash)])
        .query_async(&mut conn)
        .await
        .unwrap();
    exists == 1
}

pub async fn create_torrent_hash(request: &AnnounceRequest) {
    let client = get_client();
    let mut conn = client.get_async_connection().await.unwrap();
    if request.left == 0 {
        let _: () = conn
            .sadd(
                format!("seeders:{:?}", request.info_hash), // HACK: make sure to change this back to the actual hash.
                format!("{:?}:{}", request.ip_address, request.port),
            )
            .await
            .unwrap();
    } else {
        let _: () = conn
            .sadd(
                format!("leechers:{:?}", request.info_hash),
                format!("{:?}:{}", request.ip_address, request.port),
            )
            .await
            .unwrap();
    }
}

pub async fn get_leecher_count(request: &AnnounceRequest) -> i64 {
    let client = get_client();
    let mut conn = client.get_async_connection().await.unwrap();
    let leechers: i64 = conn
        .scard(format!("leechers:{:?}", request.info_hash))
        .await
        .unwrap();
    leechers
}

pub async fn get_seeder_count(request: &AnnounceRequest) -> i64 {
    let client = get_client();
    let mut conn = client.get_async_connection().await.unwrap();
    let seeders: i64 = conn
        .scard(format!("seeders:{:?}", request.info_hash))
        .await
        .unwrap();
    seeders
}

pub async fn get_peers(request: &AnnounceRequest) -> Peers {
    let client = get_client();
    let mut conn = client.get_async_connection().await.unwrap();

    // Setting amount of peers wanted
    let peer_amount: usize;
    if request.num_want == -1 {
        peer_amount = 80;
    } else {
        peer_amount = usize::try_from(request.num_want).unwrap();
    }

    // Getting seeders, if not enough seeders, get some leechers as well
    let mut db_peers: Vec<String> = conn
        .smembers(format!("seeders:{:?}", request.info_hash))
        .await
        .unwrap();
    if db_peers.len() < peer_amount {
        let mut db_leechers: Vec<String> = conn
            .smembers(format!("leechers:{:?}", request.info_hash))
            .await
            .unwrap();
        db_peers.append(&mut db_leechers);
    }

    // Creating the return Peers struct
    let mut return_peers: response::Peers = response::Peers { peers: Vec::new() };

    // Change the vectors of strings to vector of peers
    for db_peer in db_peers {
        let addr: Vec<&str> = db_peer.split(":").collect();
        let ip_address: u32 = addr[0].parse().unwrap();
        let port: u16 = addr[1].parse().unwrap();
        return_peers.peers.push(response::Peer { ip_address, port })
    }
    return_peers
}
