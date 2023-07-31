#[derive(Debug)]
pub struct Request{
    pub id: i64,
    pub action: i32,
    pub payload: [u8; 1484],
}
impl Request {
    pub fn from_bytes(bytes: [u8; 1496]) -> Self {
        Request {
            id: i64::from_be_bytes(bytes[0..8].try_into().unwrap()),
            action: i32::from_be_bytes(bytes[8..12].try_into().unwrap()),
            payload: bytes[12..1496].try_into().unwrap(),
        }
    }

    pub fn to_connet_request(self) -> ConnectRequest {
        ConnectRequest {
            protocol_id: self.id,
            transaction_id: i32::from_be_bytes(self.payload[0..4].try_into().unwrap()) }
    }

    pub fn to_announce_request(self) -> AnnounceRequest {
        // TODO: change to use vector of u8s instead of strings I guess
        AnnounceRequest { connection_id: self.id,
                          transaction_id: i32::from_be_bytes(self.payload[0..4].try_into().unwrap()),
                          info_hash: String::from_utf8(self.payload[4..24].to_vec()).unwrap(),
                          peer_id: String::from_utf8(self.payload[24..44].to_vec()).unwrap(),
                          downloaded: i64::from_be_bytes(self.payload[44..52].try_into().unwrap()),
                          left: i64::from_be_bytes(self.payload[52..60].try_into().unwrap()),
                          uploaded: i64::from_be_bytes(self.payload[60..68].try_into().unwrap()),
                          event: i32::from_be_bytes(self.payload[68..72].try_into().unwrap()),
                          ip_address: i32::from_be_bytes(self.payload[72..76].try_into().unwrap()),
                          key: i32::from_be_bytes(self.payload[76..80].try_into().unwrap()),
                          num_want: i32::from_be_bytes(self.payload[80..84].try_into().unwrap()),
                          port: i16::from_be_bytes(self.payload[84..86].try_into().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct ConnectRequest {
    pub protocol_id: i64,
    pub transaction_id: i32,
}

#[derive(Debug)]
pub struct AnnounceRequest {
    pub connection_id: i64,
    pub transaction_id: i32,
    pub info_hash: String,    // String
    pub peer_id: String,      // String
    pub downloaded: i64,
    pub left: i64,
    pub uploaded: i64,
    pub event: i32,             // Default 0: none, 1: completed, 2: started, 3: stopped
    pub ip_address: i32,        // Default 0
    pub key: i32,
    pub num_want: i32,          // Default -1
    pub port: i16,
}

#[derive(Debug)]
pub struct ScrapeRequest {
  pub connection_id: i64,
  pub transaction_id: i32,
  pub info_hashes: Vec<[u8; 20]>,
}
