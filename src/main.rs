use std::env;

mod torrent_meta;
mod tracker;
mod utils;
mod peer;
// step 1 parse torrent files

pub fn main()  {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let metadata = torrent_meta::TorrentMetadata::from_file(filename);
    // metadata.pretty_print();
    let mut response = tracker::get_peers(&metadata, 8080);
    let peer = &mut response.peers[0];
    peer.connect(&metadata)
}