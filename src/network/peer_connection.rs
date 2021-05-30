use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write, Error};
use std::time::Duration;
use crate::torrent_meta::TorrentMetadata;
use crate::utils::{PEER_ID, PROTOCOL, parse_big_endian};
use crate::network::message_parser::Message;
use crate::peer::Peer;

#[derive(Debug)]
pub struct PeerConnection {
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

    pub fn handshake(&mut self) {

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

    pub fn fetch_data(&mut self) -> Option<Message> {
        match self.read(4) {
            Some(length) => {
                let payload = self.read(parse_big_endian(&length.as_slice()));
                println!("Length {:?} Payload: {:?}", length, payload);
                match payload {
                    Some(data) => Some(Message::from_bytes(data)),
                    _ => None
                }
            },
            _ => {
                println!("No data received for {}", &self.peer);
                None
            }
        }
    }

    pub fn send_data(&mut self, message:Message) {
        self.stream.write_all(&message.to_bytes()).unwrap();
    }

    fn read(&mut self, bytes_to_read: u32) -> Option<Vec<u8>> {
        let mut buf = vec![];
        let stream_ref = &mut self.stream;
        let mut take = stream_ref.take(bytes_to_read as u64);
        let bytes_read = take.read_to_end(&mut buf);
        match bytes_read {
            Ok(n) => {
                if (n as u32) == bytes_to_read {
                    Some(buf)
                } else {
                    None
                }
            }
            Err(e) => {
                println!("Error reading bytes {}", e);
                None
            }
        }
    }
}