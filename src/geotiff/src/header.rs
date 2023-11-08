use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use crate::err::TIFFErrorState;
use crate::util::{ByteOrder, FromBytes};

pub fn parse_header(reader: &mut BufReader<File>) -> Result<(ByteOrder, u32), TIFFErrorState> {
    match reader.seek(SeekFrom::Start(0)) {
        Ok(..) => {
            let mut buf = [0u8; 8];
            let byte_order = match reader.read_exact(&mut buf) {
                Ok(..) => {
                    if buf[0..2] == [73, 73] {
                        ByteOrder::LittleEndian
                    } else if buf[0..2] == [77, 77] {
                        ByteOrder::LittleEndian
                    } else {
                        return Err(TIFFErrorState::UnexpectedFormat("Unexpected Header Format 1".to_string()));
                    }
                },
                Err(_) => {
                    return Err(TIFFErrorState::UnexpectedFormat("Unexpected Header Format 2".to_string()));
                }
            };
            if !match &byte_order {
                ByteOrder::LittleEndian => (buf[2..4][0] == 42) && (buf[2..4][1] == 0),
                ByteOrder::BigEndian => (buf[2..4][0] == 0) && (buf[2..4][1] == 42)
            } {
                println!("{:?}", byte_order);
                println!("{:?} {:?}", buf[2..4][0], buf[2..4][1]);
                // Is not tiff!
                return Err(TIFFErrorState::UnexpectedFormat("Unexpected Header Format 3".to_string()));
            }

            return Ok((byte_order.clone(), u32::from_bytes(&buf[4..8], &byte_order, )));
        },
        Err(_) => Err(TIFFErrorState::UnexpectedFormat("Unexpected Header Format 4".to_string()))
    }
}