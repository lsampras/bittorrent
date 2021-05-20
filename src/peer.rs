use std::net::{IpAddr, Ipv4Addr};
use std::net::{TcpStream, SocketAddr};
use std::fmt::Formatter;
use std::io::{Read, Write, Error, ErrorKind};
use std::time::Duration;
use crate::torrent_meta::TorrentMetadata;
use crate::utils::{PEER_ID, PROTOCOL, parse_big_endian};

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


struct PeerConnection {
    peer: Peer,
    stream: TcpStream,
    torrent: TorrentMetadata
}

impl PeerConnection {

    pub fn new(peer: Peer, torrent_meta:TorrentMetadata) -> Result<PeerConnection, Error> {
        println!("Connecting to {}...", &peer);
        let addr = SocketAddr::new(peer.ip, peer.port);
        match TcpStream::connect_timeout(&addr, Duration::new(10,0)) {
            Ok(stream_obj) => {
                println!("Connected successfully to {}", &peer);

                Ok(PeerConnection {
                    peer: peer,
                    stream: stream_obj,
                    torrent: torrent_meta
                })
            }
            Err(e) => panic!("Failed to create a PeerConnection : {}", e)
        }
    }

    fn handshake(&mut self) {

        let mut message = vec![];
        message.push(PROTOCOL.len() as u8);
        message.extend(PROTOCOL.bytes());
        message.extend(vec![0;8].into_iter());
        message.extend(self.torrent.info_hash.iter().cloned());
        message.extend(PEER_ID.bytes());
        self.stream.write_all(&message).unwrap();

        let pstrlen = self.read(1).unwrap();
        let _pstr = self.read(pstrlen[0] as u32).unwrap();
        let _reserved = self.read(8).unwrap();
        let _info_hash = self.read(20).unwrap();
        let _peer_id = self.read(20).unwrap();
        println!("Received handshake");
    }

    fn fetch_data(&mut self) {

        match self.read(4) {
            Ok(length) => {
                let payload = self.read(parse_big_endian(&length.as_slice()));
                println!("Length {:?} Payload: {:?}", length, payload);
            }
            Err(_) => println!("No data received for {}", &self.peer)
        }
    }

    fn read(&mut self, bytes_to_read: u32) -> Result<Vec<u8>, Error> {
        let mut buf = vec![];
        let stream_ref = &mut self.stream;
        let mut take = stream_ref.take(bytes_to_read as u64);
        let bytes_read = take.read_to_end(&mut buf);
        match bytes_read {
            Ok(n) => {
                if (n as u32) == bytes_to_read {
                    Ok(buf)
                } else {
                    Err(Error::new(ErrorKind::Other, "No data received"))
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}