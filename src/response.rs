#[derive(Debug)]
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
    pub ip_address: i32,
    pub port: i16,
}

#[derive(Debug)]
pub struct AnnounceResponse {
    pub transaction_id: i32,
    pub interval: i32,
    pub leechers: i32,
    pub seeders: i32,
    pub peers: Vec<Peer>,
}
impl AnnounceResponse {
    pub fn to_bytes(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend((1 as i32).to_be_bytes());
        out.extend(self.transaction_id.to_be_bytes());
        out.extend(self.interval.to_be_bytes());
        out.extend(self.leechers.to_be_bytes());
        out.extend(self.seeders.to_be_bytes());
        for peer in self.peers {
            out.extend(peer.ip_address.to_be_bytes());
            out.extend(peer.port.to_be_bytes());
        }
        out
    }
}
