# bittorrent
BitTorrent protocol implemented in rust. this is a simple barebones implementation for the sake of learning

### TODO:
- ~~Parse Torrent Metadata files~~
- ~~Fetch File Info from the Tracker~~
- ~~Fetch pieces from Peers~~
- Use Worker threads for fetching peices
- ~~Handle Peer signals (choke/unchoke/Have/Want)~~
- Manage and assign proper pieces to be fetched for each worker/peer
- Manage priority of pieces/peers
