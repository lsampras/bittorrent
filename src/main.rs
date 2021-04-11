use std::env;

mod torrent_meta;
mod tracker;
mod utils;
// step 1 parse torrent files

pub fn main()  {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let metadata = torrent_meta::TorrentMetadata::from_file(filename);
    // metadata.pretty_print();
    tracker::get_peers(metadata,8080)
}