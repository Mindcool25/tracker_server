use std::net::Ipv4Addr;

#[derive(Debug, Copy, Clone)]
pub struct ConnectResponse {
    pub transaction_id: i32,
    pub connection_id: i64,
}
impl ConnectResponse {
    pub fn to_bytes(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend((0 as i32).to_be_bytes());
        out.extend(self.transaction_id.to_be_bytes());
        out.extend(self.connection_id.to_be_bytes());
        out
    }
}

#[derive(Debug)]
pub struct Peer {
    pub ip_address: Ipv4Addr,
    pub port: u16,
}

#[derive(Debug)]
pub struct Peers {
    pub peers: Vec<Peer>,
}

#[derive(Debug)]
pub struct AnnounceResponse {
    pub transaction_id: i32,
    pub interval: i32,
    pub leechers: i32,
    pub seeders: i32,
    pub peers: Peers,
}
impl AnnounceResponse {
    pub fn to_bytes(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend((1 as i32).to_be_bytes());
        out.extend(self.transaction_id.to_be_bytes());
        out.extend(self.interval.to_be_bytes());
        out.extend(self.leechers.to_be_bytes());
        out.extend(self.seeders.to_be_bytes());
        for peer in self.peers.peers {
            out.extend(peer.ip_address.octets());
            out.extend(peer.port.to_be_bytes());
        }
        out
    }
}

#[derive(Debug)]
pub struct TorrentStats {
    pub complete: i32,
    pub downloaded: i32,
    pub incomplete: i32,
}

#[derive(Debug)]
pub struct ScrapeResponse {
    pub transaction_id: i32,
    pub torrents: Vec<TorrentStats>,
}
impl ScrapeResponse {
    pub fn to_bytes(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend((2 as i32).to_be_bytes());
        out.extend(self.transaction_id.to_be_bytes());
        for torrent in self.torrents {
            out.extend(torrent.complete.to_be_bytes());
            out.extend(torrent.downloaded.to_be_bytes());
            out.extend(torrent.incomplete.to_be_bytes());
        }
        out
    }
}

#[derive(Debug)]
pub struct ErrorResponse {
    pub transaction_id: i32,
    pub message: String,
}
impl ErrorResponse {
    pub fn to_bytes(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend((3 as i32).to_be_bytes());
        out.extend(self.transaction_id.to_be_bytes());
        out.extend(self.message.into_bytes());
        out
    }
}
