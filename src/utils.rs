use crate::torrent_meta::TorrentMetadata;
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};


pub fn parameterize(parameters: Vec<(&str, &str)>) -> String {
    let query_params: Vec<String> = parameters.iter()
            .map(|&kv| format!("{}={}", kv.0, kv.1))
            .collect();

    query_params.join("&")
}

pub fn get_formatted_url(metadata: &TorrentMetadata, port: &u16) -> String {

    let peer_id = "-TR2940-k8hj0wgej6ch";
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