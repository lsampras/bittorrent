
use crate::torrent_meta::TorrentMetadata;
use crate::utils::get_formatted_url;

pub fn get_peers(metadata: TorrentMetadata, port: u16) {

    let query_url = get_formatted_url(&metadata, &port);
    println!("{:?}", query_url);

    let body = reqwest::blocking::get(query_url).unwrap().text().unwrap();

    println!("{:?}", body.as_bytes());
}