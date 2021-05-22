use std::env;

mod torrent_meta;
mod tracker;
mod utils;
mod peer;
mod network;


pub fn test() {
    let test: Vec<u8> = vec![20,1,2,3,4,5,6,7];
    let mut bitfield = Vec::<bool>::new();
    println!("{:?}", test.into_iter().map(|data_byte| {
        vec![
            data_byte & (1 << 0) != 0,
            data_byte & (1 << 1) != 0,
            data_byte & (1 << 2) != 0,
            data_byte & (1 << 3) != 0,
            data_byte & (1 << 4) != 0,
            data_byte & (1 << 5) != 0,
            data_byte & (1 << 6) != 0,
            data_byte & (1 << 7) != 0,
        ]
    }).flatten().collect::<Vec<bool>>());
    panic!();
}

pub fn main()  {
    test();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let metadata = torrent_meta::TorrentMetadata::from_file(filename);
    // metadata.pretty_print();
    let mut response = tracker::get_peers(&metadata, 8080);
    let peer = &mut response.peers[0];
    peer.connect(metadata)
}