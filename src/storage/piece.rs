
use crypto::sha1::Sha1;
use crypto::digest::Digest;

#[derive(Debug)]
pub struct Piece {
    pub piece_length: u64,
    data_blocks: Vec<Vec<u8>>,
    blocks_downloaded: Vec<bool>,
    requested_blocks_index: Vec<u32>,
    is_complete: bool,
    hash: Vec<u8>,
    pub index: u32,
}

static BLOCK_SIZE : u64 = 16 * 1024;

impl Piece {
    pub fn new(length: u64, index: u32, hash: Vec<u8>) -> Piece {
        let num_blocks = length/BLOCK_SIZE;
        Piece {
            piece_length: length,
            data_blocks: vec![vec![]; num_blocks as usize],
            blocks_downloaded: vec![false; num_blocks as usize],
            requested_blocks_index: vec![],
            is_complete: false,
            hash: hash,
            index: index
        }
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn store_data(&mut self, offset: u64, data: Vec<u8>) {
        // TODO: validate hash
        if data.len() == BLOCK_SIZE as usize {
            let block_index = offset/BLOCK_SIZE;
            self.data_blocks[block_index as usize] = data;
            self.blocks_downloaded[block_index as usize] = true;
        }
        for downloaded in self.blocks_downloaded.iter() {
            if !downloaded {
                return
            }
        }
        self.is_complete = true;
        println!("Verifying hash for piece: {}", self.index);
        let data: Vec<u8> = self.data_blocks.clone().into_iter().flatten().collect();
        let mut hasher = Sha1::new();
        hasher.input(&data[..]);
        let mut hex: Vec<u8> = vec![0; 20];
        hasher.result(&mut hex);
        if hex == self.hash {
            println!("Hash verified Succesfully for piece {}", self.index);
        } else {
            println!("Hash mismatch!\n expected:\t{:?}\n actual:\t{:?}", self.hash, hex);
        }
    }

    pub fn get_next_block_request(&self) -> (u32, u64, u64) {
        if self.is_complete {
            panic!("Piece {} is already complete", self.index)
        }
        for (i, downloaded) in self.blocks_downloaded.iter().enumerate() {
            if !downloaded {
                return (self.index, (i as u64)*BLOCK_SIZE, BLOCK_SIZE)
            }
        }
        panic!("Free block not found")
    }

}