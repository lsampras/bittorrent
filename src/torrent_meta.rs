
use std::io::prelude::*;
use std::fs;
use serde_derive::{Serialize, Deserialize};
use serde_bencode;
use serde_bytes::ByteBuf;

use crypto::sha1::Sha1;
use crypto::digest::Digest;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Node(String, i64);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct File {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct Info {
    name: String,
    pieces: ByteBuf,
    #[serde(rename="piece length")]
    piece_length: i64,
    #[serde(default)]
    md5sum: Option<String>,
    #[serde(default)]
    length: Option<i64>,
    #[serde(default)]
    files: Option<Vec<File>>,
    #[serde(default)]
    private: Option<u8>,
    #[serde(default)]
    path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename="root hash")]
    root_hash: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct TorrentMetadata {
    info: Info,
    #[serde(default)]
    announce: Option<String>,
    #[serde(default)]
    nodes: Option<Vec<Node>>,
    #[serde(default)]
    encoding: Option<String>,
    #[serde(default)]
    httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename="announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename="creation date")]
    creation_date: Option<i64>,
    #[serde(rename="comment")]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename="created by")]
    created_by: Option<String>,
    info_hash: Vec<u8>
}


// #[derive(Debug, Deserialize, Serialize)]
// struct File {
//     path: Vec<String>,
//     length: i64,
//     #[serde(default)]
//     md5sum: Option<String>,
// }

// #[derive(Debug, Default, Deserialize, Serialize)]
// pub struct Info {
//     name: String,
//     pieces: ByteBuf,
//     #[serde(rename="piece length")]
//     piece_length: i64,
//     #[serde(default)]
//     md5sum: Option<String>,
//     #[serde(default)]
//     pub length: Option<i64>,
//     #[serde(default)]
//     files: Option<Vec<File>>,
//     #[serde(default)]
//     private: Option<u8>,
//     #[serde(default)]
//     path: Option<Vec<String>>,
    // #[serde(default)]
    // #[serde(rename="root hash")]
    // root_hash: Option<String>,
// }

// // the Deserialize derive is provided by serde, it causes the code
// // to be generated that is needed for deserialization
// #[derive(Default, Debug, Deserialize)]
// // this tells serde you want to use kebab-case, which is 
// // with dashes instead of underscores for field names
// #[serde(rename_all = "kebab-case")]
// // this tells serde to default values to the empty case if not provided
// #[serde(default)]
// pub struct Torrent {
//     pub info: Info,
//     pub announce: String,
//     // nodes: Option<Vec<Node>>,
//     encoding: Option<String>,
//     httpseeds: Vec<String>,
//     #[serde(rename="announce-list")]
//     announce_list: Vec<Vec<String>>,
//     #[serde(rename="creation date")]
//     creation_date: i64,
//     #[serde(rename="comment")]
//     comment: String,
//     #[serde(rename="created by")]
//     created_by: String,
//     pub info_hash: Vec<u8>,
// }

impl TorrentMetadata {

    pub fn from_file(filename: &String) -> TorrentMetadata {
        let mut f = fs::File::open(filename).unwrap();
        let mut s = Vec::new();
        f.read_to_end(&mut s).unwrap();
        let mut decoded: TorrentMetadata = serde_bencode::from_bytes(&s).unwrap();
        decoded.info_hash = create_info_hash(&decoded.info);
        decoded
    }

    pub fn pretty_print(&self) {
        println!("name:\t\t{}", self.info.name);
        println!("hash:\t\t{:x?}", self.info_hash);
        println!("announce:\t{:?}", self.announce);
        println!("nodes:\t\t{:?}", self.nodes);
        if let &Some(ref al) = &self.announce_list {
            for a in al {
                println!("announce list:\t{}", a[0]);
            }
        }
        println!("httpseeds:\t{:?}", self.httpseeds);
        println!("creation date:\t{:?}", self.creation_date);
        println!("comment:\t{:?}", self.comment);
        println!("created by:\t{:?}", self.created_by);
        println!("encoding:\t{:?}", self.encoding);
        println!("piece length:\t{:?}", self.info.piece_length);
        println!("private:\t{:?}", self.info.private);
        println!("root hash:\t{:?}", self.info.root_hash);
        println!("md5sum:\t\t{:?}", self.info.md5sum);
        println!("path:\t\t{:?}", self.info.path);
        if let &Some(ref files) = &self.info.files {
            for f in files {
                println!("file path:\t{:?}", f.path);
                println!("file length:\t{}", f.length);
                println!("file md5sum:\t{:?}", f.md5sum);
            }
        }
    }
}


fn create_info_hash(info: &Info) -> Vec<u8> {
    let info_raw = serde_bencode::to_bytes(info).unwrap();
    let mut hasher = Sha1::new();
    hasher.input(&info_raw[..]);
    let mut hex: Vec<u8> = vec![0; 20];
    hasher.result(&mut hex);
    hex
}
