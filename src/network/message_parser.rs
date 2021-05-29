use crate::utils::{parse_big_endian, convert_u8_to_bits, u32_to_big_endian};

pub enum Message {
    Choke,
    UnChoke,
    Interested,
    NotInterested,
    Have(u32),
    Bitfield(Vec<bool>),
    Request(u32, u32, u32),
    Piece(u32, u32, Vec<u8>),
    Cancel(u32),
}

impl Message {
    pub fn from_bytes(data : Vec<u8>) -> Message {
        let (msg_id, msg_payload) = data.split_at(1);

        match msg_id {
            [0] => Message::Choke,
            [1] => Message::UnChoke,
            [2] => Message::Interested,
            [3] => Message::NotInterested,
            [4] => Message::Have(parse_big_endian(&msg_payload[0..4])),
            [5] => {
                Message::Bitfield(msg_payload.into_iter()
                                  .map(|data_byte| convert_u8_to_bits(data_byte))
                                  .flatten().collect::<Vec<bool>>())
            },
            [6] => Message::Request(
                parse_big_endian(&msg_payload[0..4]),
                parse_big_endian(&msg_payload[4..8]),
                parse_big_endian(&msg_payload[8..12]),
            ),
            [7] => Message::Piece(
                parse_big_endian(&msg_payload[0..4]),
                parse_big_endian(&msg_payload[4..8]),
                msg_payload[8..].to_owned()
            ),
            [8] => {
                println!(" Cancel Request parsed {:?}", &msg_payload);
                Message::Cancel(parse_big_endian(&msg_payload[0..4]))
            },
            _ => panic!("Unsupported msg_type {:?} with data {:?}", msg_id, data)
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Message::Request(piece_index, offset, length) => {
                let mut data: Vec<u8> = vec![6];
                data.extend(u32_to_big_endian(piece_index));
                data.extend(u32_to_big_endian(offset));
                data.extend(u32_to_big_endian(length));
                data
            }
            _ => panic!("Serialize not implemented for")
        }
    }
}

