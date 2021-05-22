
use crate::peer::Peer;
use crate::torrent_meta::TorrentMetadata;
use crate::tracker::TrackerResponse;
#[derive(Debug)]
pub struct PeerState {
    pub peer_id: u8
    pub peer: Peer
    pub mut have: Vec<bool>,
    pub mut choked: bool,
    pub mut interested: bool,
}

#[derive(Debug)]
pub struct TorrentState {
    torrent_meta: TorrentMetadata,
    tracker_info: TrackerResponse
}

