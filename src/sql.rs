use sqlite;
use std::net::SocketAddr;

use crate::{requests, response};

pub struct Database {
    pub url: String,
    pub connection: sqlite::Connection,
}
impl Database {
    pub fn new(url: String) -> Self {
        let newdb: Database = Database {
            url: url.clone(),
            connection: sqlite::open(url).unwrap(),
        };
        let query = "CREATE TABLE IF NOT EXISTS torrents (
                        hash TEXT,
                        leechers INTEGER
                        seeders INTEGER
                        downloaded INTEGER
                        peers STRING);";
        newdb.connection.execute(query).unwrap();
        newdb
    }
    pub fn handle_announce(&self, request: requests::AnnounceRequest, addr: &SocketAddr) -> Vec<u8> {
        let query = "SELECT * FROM torrents WHERE hash = :hash";
        for row in self.connection
                       .prepare(query)
                       .unwrap()
                       .into_iter()
                       .bind((":hash", request.info_hash.as_str()))
                       .unwrap()
                       .map(|row| row.unwrap())
        {
            if request.ip_address == 0 {
                let ann_ip = addr.ip();
            }
            else {
                let ann_ip = request.ip_address;
            }
            let mut peers: response::Peers = serde_bencode::from_str(row.read::<&str, _>("peers")).unwrap();
            peers.peers.push(response::Peer{ip_address: ann_ip, port: addr.port()});
            return response::AnnounceResponse {
                transaction_id: request.transaction_id,
                interval: 5,
                leechers: i32::try_from(row.read::<i64, _>("leechers")).unwrap(),
                seeders: i32::try_from(row.read::<i64, _>("seeders")).unwrap(),
                peers,
            }.to_bytes();
        }
        println!("Hash not found");
        Vec::new()
    }

    pub fn update_peers(&self, new_peers: response::Peers, hash: String) {
        let query = format!("UPDATE torrents
                            SET peers = {}
                            WHERE hash = :hash", serde_bencode::to_string(&new_peers).unwrap());
        let mut statement = self.connection.prepare(query).unwrap();
        statement.bind((":hash", hash.as_str())).unwrap();
    }
}
