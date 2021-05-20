use std::net::{IpAddr, Ipv4Addr};
use std::net::{TcpStream, SocketAddr};
use std::fmt::Formatter;
use std::io::{Read, Write, Error, ErrorKind};

use crate::torrent_meta::TorrentMetadata;
use crate::utils::{PEER_ID, PROTOCOL};

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

    pub fn connect(&mut self, torrent_mutex: &TorrentMetadata) {
        println!("Connecting to {}...", &self);
        let addr = SocketAddr::new(self.ip, self.port);
        match TcpStream::connect(&addr) {
            Ok(mut stream_obj) => {
                println!("Connected successfully to {}", &self);

                let stream = &mut stream_obj;
                let mut message = vec![];
                message.push(PROTOCOL.len() as u8);
                message.extend(PROTOCOL.bytes());
                message.extend(vec![0;8].into_iter());
                message.extend(torrent_mutex.info_hash.iter().cloned());
                message.extend(PEER_ID.bytes());
                stream.write_all(&message).unwrap();
                println!("Sent handshake");

                let pstrlen = self.read_n(1, stream).unwrap();
                let _pstr = self.read_n(pstrlen[0] as u32, stream).unwrap();
                let _reserved = self.read_n(8, stream).unwrap();
                let _info_hash = self.read_n(20, stream).unwrap();
                let _peer_id = self.read_n(20, stream).unwrap();
                println!("Received handshake");
            }
            _ => println!("Failed to connect")
        }
    }

    fn read_n(&mut self, bytes_to_read: u32, stream: &mut TcpStream) -> Result<Vec<u8>, Error> {
        let mut buf = vec![];
        let mut take = stream.take(bytes_to_read as u64);
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