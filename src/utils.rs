use crate::torrent_meta::TorrentMetadata;
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

// TODO: Use a random string for PEER ID
pub const PEER_ID : &str = "-TR2940-k9hj0wfej5ch";
pub const PROTOCOL: &'static str = "BitTorrent protocol";

pub fn parameterize(parameters: Vec<(&str, &str)>) -> String {
    let query_params: Vec<String> = parameters.iter()
            .map(|&kv| format!("{}={}", kv.0, kv.1))
            .collect();

    query_params.join("&")
}

pub fn get_formatted_url(metadata: &TorrentMetadata, port: &u16) -> String {

    let peer_id = PEER_ID;
    let uploaded = 0.to_string();
    let downloaded = 0.to_string();
    let left = metadata.info.length.unwrap().to_string();
    let port_str = &port.to_string();
    let compact = 1.to_string();
    let base_url = &metadata.announce.as_ref().unwrap();
    const QUERY: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');
    let percent_encoded_hash: String = percent_encode(&metadata.info_hash, QUERY).collect();

    let params: Vec<(&str, &str)> = vec![
        ("info_hash", percent_encoded_hash.as_ref()),
        ("peer_id", peer_id),
        ("port", port_str),
        ("uploaded", uploaded.as_ref()),
        ("downloaded", downloaded.as_ref()),
        ("left", left.as_ref()),
        ("compact", compact.as_ref()),
        ("event", "started")
    ];

    let query_params = parameterize(params);
    format!("{}?{}", base_url, query_params)
}

pub fn parse_big_endian(num: &[u8]) -> u32 {
    let mut buf = Cursor::new(&num);
    buf.read_u32::<BigEndian>().unwrap()
}

pub fn u32_to_big_endian(integer: &u32) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.write_u32::<BigEndian>(integer.to_owned()).unwrap();
    bytes
}

pub fn convert_u8_to_bits(data_byte: &u8) -> Vec<bool>{
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
}