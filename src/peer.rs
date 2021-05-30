use std::net::{IpAddr, Ipv4Addr};
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Peer {
    pub ip: IpAddr,
    pub port: u16
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}:{}", self.ip, self.port)
    }
}


impl Peer {
    pub fn from_bytes(v: &[u8]) -> Self {
        let ip = IpAddr::V4(Ipv4Addr::new(v[0], v[1], v[2], v[3]));
        let port = v[4] as u16 * 256 + v[5] as u16;
        Peer {
            ip: ip,
            port: port
        }
    }
}
