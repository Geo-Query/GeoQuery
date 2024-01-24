use std::io::{SeekFrom};
use crate::header::HeaderErrorState::UnexpectedMagicNumber;
use crate::TIFFErrorState;
use crate::util::{ByteOrder, FromBytes};

#[derive(Debug)]
pub enum HeaderErrorState {
    UnexpectedByteOrder([u8; 2]),
    UnexpectedMagicNumber([u8; 2]),
    InvalidLength(usize)
}

pub fn parse_header(buffer: &[u8]) -> Result<(ByteOrder, SeekFrom), TIFFErrorState> {
    if buffer.len() != 8 {
        return Err(TIFFErrorState::HeaderError(HeaderErrorState::InvalidLength(buffer.len())));
    }

    let byte_order = match buffer[0..2] {
        [73, 73] => ByteOrder::LittleEndian,
        [77, 77] => ByteOrder::BigEndian,
        _ => return Err(TIFFErrorState::HeaderError(HeaderErrorState::UnexpectedByteOrder([buffer[0], buffer[1]])))
    };

    let magic_numbers = &buffer[2..4];
    if !match byte_order {
        ByteOrder::LittleEndian => (magic_numbers[0] == 42) && (magic_numbers[1] == 0),
        ByteOrder::BigEndian => (magic_numbers[0] == 0) && (magic_numbers[1] == 42)
    } {
        return Err(TIFFErrorState::HeaderError(UnexpectedMagicNumber([magic_numbers[0], magic_numbers[1]])));
    }
    let ifd_offset = SeekFrom::Start(
        u32::from_bytes(&buffer[4..8], &byte_order) as u64
    );
    return Ok((byte_order, ifd_offset));
}