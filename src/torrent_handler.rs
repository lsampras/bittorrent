
use crate::peer::Peer;
use crate::torrent_meta::TorrentMetadata;
use crate::tracker::TrackerResponse;
use crate::storage::piece::Piece;
use crate::network::message_parser::Message;
use crate::network::peer_connection::PeerConnection;
use std::sync::{Arc, Mutex};

use std::{thread, time};

#[derive(Debug)]
pub struct PeerState {
    pub peer_id: usize,
    pub peer: Peer,
    pub connection: PeerConnection,
    pub have: Vec<bool>,
    pub choked: bool,
    pub interested: bool
    // temporary variable later to be replaced with a pipe or mutex
    // pub torrent_state: &'static TorrentState
}

#[derive(Debug)]
pub struct PieceHandler {
    have: Vec<bool>,
    want: Vec<bool>,
    pub pieces: Vec<Piece>
}

#[derive(Debug)]
pub struct TorrentState {
    torrent_meta: TorrentMetadata,
    tracker_info: TrackerResponse,
    pub peers: Vec<PeerState>,
    pub piece_handler: Arc<Mutex<PieceHandler>>
}

impl PieceHandler {
    pub fn create(hashes: &Vec<Vec<u8>>, piece_length: u64) -> Self {

        PieceHandler {

            pieces : hashes.iter().enumerate().map(|(idx, hash)| {
                Piece::new(piece_length, idx as u32, hash.to_vec())
            }).collect(),
            want: vec![true; hashes.len()],
            have: vec![false; hashes.len()],
        }
    }

    pub fn store_data(&mut self, piece_index:u32, offset:u32, data:Vec<u8>) {

        self.pieces[piece_index as usize].store_data(offset.into(), data);
    }

    pub fn get_block_request(&self) -> Option<(u32, u64, u64)> {
        for i in self.pieces.iter() {
            if !i.is_complete() {
                return Some(i.get_next_block_request());
            }
        }
        None
    }
}

impl TorrentState {

    pub fn create(torrent_meta: TorrentMetadata, tracker_res : TrackerResponse) -> Self {

        let piece_mutex = Arc::new(Mutex::new(PieceHandler::create(&torrent_meta.info.pieces_hash.clone().unwrap(), torrent_meta.info.piece_length)));
        let mut torrent_state = TorrentState {
            torrent_meta: torrent_meta,
            tracker_info: tracker_res,
            peers: vec![],
            piece_handler: piece_mutex
        };
        // torrent_state.init_peers();
        torrent_state
    }

    fn init_peers(&mut self, pieces_len: usize) {

        self.peers = self.tracker_info.peers[0..1].to_vec().iter().enumerate().map(|(idx, peer)| {
            PeerState::new(peer.to_owned(), self.torrent_meta.clone(), idx, pieces_len)
        }).collect();
    }

    // pub fn run(&mut self) {
    //     let peer = self.peers[0];
    //     peer.run();
    // }
}

impl PeerState {

    pub fn new(peer: Peer, torrent_meta: TorrentMetadata, id: usize, num_pieces: usize) -> Self {

        let connection = PeerConnection::new(peer, torrent_meta).unwrap();
        PeerState {
            peer_id: id,
            peer: peer,
            connection: connection,
            have: vec![false; num_pieces],
            choked: true,
            interested: false
        }
    }

    pub fn run(&mut self, piece_handler: Arc<Mutex<PieceHandler>>) {
        self.connection.handshake();
        let mut stop_flag = false;
        while !stop_flag {
            println!("Running loop for {}:{}", self.peer_id, self.peer);
            match self.connection.fetch_data() {
                Some(message) => {
                    match message {
                        Message::Choke => {
                            println!("{:?} has choked us!!", self.peer);
                            self.choked = true;
                        },
                        Message::UnChoke => {
                            println!("{:?} has unchoked us!!", self.peer);
                            self.choked = false;
                            if !self.choked {

                                match piece_handler.lock().unwrap().get_block_request() {
                                    Some((index, offset, len)) => {
    
                                        println!("Requesting {}  for piece {}, offset {}, len{}", self.peer, index, offset, len);
                                        self.connection.send_data(Message::Request(index, offset as u32, len as u32));
                                    },
                                    None => {stop_flag = true;}
                                }
                            }
                        },
                        Message::Interested => {
                            println!("{:?} is interested", self.peer);
                        },
                        Message::NotInterested => {
                            println!("{:?} is not interested", self.peer);
                        },
                        Message::Have(index) => {
                            println!("Received Have for {}", index);
                            self.have[index as usize] = true;
                        },
                        Message::Bitfield(have) => {
                            self.have = have;
                            println!("Received Bit Field");
                            self.connection.send_data(Message::Interested);
                        },
                        Message::Request(piece_index, offset, length) => {
                            // TODO: implement this
                            println!("{} has requested for a piece {}, offset {}, len{}",
                                     self.peer, piece_index, offset, length);
                        },
                        Message::Piece(piece_index, offset, block) => {
                            // TODO: Use a multithread safe approach here
                            println!("{} has sent a piece {}, offset {}, len{}", self.peer, piece_index, offset, &block.len());
                            piece_handler.lock().unwrap().store_data(piece_index, offset, block);

                            // TODO: request next piece
                            if !self.choked {

                                match piece_handler.lock().unwrap().get_block_request() {
                                    Some((index, offset, len)) => {
    
                                        println!("Requesting {}  for piece {}, offset {}, len{}", self.peer, index, offset, len);
                                        self.connection.send_data(Message::Request(index, offset as u32, len as u32));
                                    },
                                    None => {stop_flag = true;}
                                }
                            }
                        },
                        Message::Cancel(_have) => {
                            // TODO; implement this
                            println!("Received a cancel from {}", self.peer)
                        }
                    }
                },
                _ => {
                    println!("Sleeping");
                    let sleep_time = time::Duration::new(2, 0);
                    thread::sleep(sleep_time);
                }
            }
        }
    }
}