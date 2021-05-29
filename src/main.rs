use std::env;

mod torrent_meta;
mod tracker;
mod utils;
mod peer;
mod network;
mod torrent_handler;
mod storage;


pub fn test() {
    let test: Vec<u8> = vec![20,1,2,3,4,5,6,7];
    let mut bitfield = vec![false; 20];
    println!("{:?}", bitfield);
    panic!();
}

pub fn main()  {
    // test();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let metadata = torrent_meta::TorrentMetadata::from_file(filename);
    metadata.pretty_print();
    let mut response = tracker::get_peers(&metadata, 8080);
    let mut handler = torrent_handler::TorrentState::create(metadata.clone(), response.clone());
    let mut peer_state = torrent_handler::PeerState::new(response.peers[0], metadata, 0, handler.pieces.len());
    peer_state.run(&mut handler);
    // let peer = &mut response.peers[0];
    // peer.connect(metadata)
}