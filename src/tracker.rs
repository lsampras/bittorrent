use crate::torrent_meta::TorrentMetadata;
use crate::utils::get_formatted_url;
use crate::peer::Peer;
use serde_derive::{Serialize, Deserialize};
use serde_bencode;

use std::str;
use serde_bytes::ByteBuf;
use reqwest::header::{HeaderMap, HeaderValue, CONNECTION};


#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
struct TrackerResponseRaw {
    #[serde(default)]
    pub interval: Option<u32>,
    #[serde(default)]
    pub peers: ByteBuf
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TrackerResponse {
    pub interval: u32,
    pub peers: Vec<Peer>
}

pub fn get_peers(metadata: &TorrentMetadata, port: u16) -> TrackerResponse {

    let query_url = get_formatted_url(&metadata, &port);
    // println!("{:?}", query_url);

    let mut headers = HeaderMap::new();
    headers.insert(CONNECTION, HeaderValue::from_static("close"));

    let client = reqwest::blocking::Client::new();
    let body = client.get(query_url).headers(headers).send().unwrap().bytes().unwrap();
    let tracker_response: TrackerResponseRaw = serde_bencode::de::from_bytes(&body).unwrap();

    let mut parsed_peers: Vec<Peer> = Vec::new();
    for i in 0..(tracker_response.peers.len() / 6) {
        let offset = i * 6;
        let peer = Peer::from_bytes(&tracker_response.peers[offset..offset+6]);
        parsed_peers.push(peer);
    }
    let tracker_res = TrackerResponse {
        interval: tracker_response.interval.unwrap(),
        peers: parsed_peers
    };
    tracker_res
}