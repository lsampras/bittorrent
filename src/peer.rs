use std::net::{IpAddr, Ipv4Addr};
use std::net::{TcpStream, SocketAddr};
use std::fmt::Formatter;
use std::io::{Read, Write, Error, ErrorKind};
use std::time::Duration;
use crate::torrent_meta::TorrentMetadata;
use crate::utils::{PEER_ID, PROTOCOL, parse_big_endian};
use crate::network::message_parser::Message;
use crate::network::peer_connection::PeerConnection;

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

    pub fn connect(&mut self, torrent_meta: TorrentMetadata) {
        let mut peer_connection = PeerConnection::new(self.to_owned(), torrent_meta).unwrap();
        &mut peer_connection.handshake();

        // This won't work for now since we need to keep asking the peer for new pieces
        let mut timer = 0;
        while timer < 50 {
            peer_connection.fetch_data();
            timer = timer + 1;
        }
    }

}
