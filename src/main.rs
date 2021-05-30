use std::env;

mod torrent_meta;
mod tracker;
mod utils;
mod peer;
mod network;
mod torrent_handler;
mod storage;

pub fn main()  {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let metadata = torrent_meta::TorrentMetadata::from_file(filename);
    metadata.pretty_print();
    let response = tracker::get_peers(&metadata, 8080);
    let mut handler = torrent_handler::TorrentState::create(metadata.clone(), response.clone());
    let mut peer_state = torrent_handler::PeerState::new(response.peers[0], metadata, 0, handler.pieces.len());
    peer_state.run(&mut handler);
    // let peer = &mut response.peers[0];
    // peer.connect(metadata)
}