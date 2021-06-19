use std::{env, thread};

mod torrent_meta;
mod tracker;
mod utils;
mod peer;
mod network;
mod torrent_handler;
mod storage;

pub fn main()  {
    let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    let filename = &String::from("test-debian.torrent");

    let metadata = torrent_meta::TorrentMetadata::from_file(filename);
    metadata.pretty_print();
    let response = tracker::get_peers(&metadata, 8080);
    let mut handler = torrent_handler::TorrentState::create(metadata.clone(), response.clone());
    let mut peer_state = torrent_handler::PeerState::new(response.peers[0], metadata.clone(), 0, handler.piece_handler.lock().unwrap().pieces.len());
    let mut peer_state2 = torrent_handler::PeerState::new(response.peers[1], metadata, 1, handler.piece_handler.lock().unwrap().pieces.len());
    let temp_handler1 = handler.piece_handler.clone();
    let temp_handler2 = handler.piece_handler.clone();
    let thread1 = thread::spawn(move || {
        peer_state.run(temp_handler1);
    });
    let thread2 = thread::spawn(move || {
        peer_state2.run(temp_handler2);
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
    // peer_state.run(handler.piece_handler);
    // let peer = &mut response.peers[0];
    // peer.connect(metadata)
}