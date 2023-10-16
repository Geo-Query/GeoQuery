use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use crate::parsing::{Descriptor, ParsingErrorState};
use crate::spatial::Region;
use crate::tiff::ByteOrder::{BigEndian, LittleEndian};
use crate::tiff::TifErrorState::InvalidHeader;

#[derive(Debug)]
enum ByteOrder {
    LittleEndian,
    BigEndian
}

enum TifErrorState {
    InvalidHeader
}

fn parse_header(reader: &mut BufReader<File>) -> Result<(ByteOrder, u32), TifErrorState> {
    match reader.seek(SeekFrom::Start(0)) {
        Ok(..) => {},
        Err(_) => {
            return Err(TifErrorState::InvalidHeader);
        }
    }
    let mut byte_order_buf= [0u8; 2];
    let byte_order: ByteOrder;
    match reader.read_exact(&mut byte_order_buf) {
        Ok(..) => {},
        Err(_) => {
            return Err(TifErrorState::InvalidHeader);
        }
    }
    if byte_order_buf[0] == byte_order_buf[1] {
        if byte_order_buf[0] == 73 {
            byte_order = LittleEndian;
        } else if byte_order_buf[0] == 77 {
            byte_order = BigEndian;
        } else {
            eprintln!("Invalid byte-order! Is this a valid TIFF file?");
            eprintln!("Byte order bytes: {:?}", byte_order_buf);
            panic!();
        }
    } else {
        eprintln!("Byte order bytes unequal! Is this a valid TIFF file?");
        eprintln!("Byte order bytes: {:?}", byte_order_buf);
        panic!();
    }
    println!("Byte Order: {:?}", byte_order);
    let mut is_tiff = [0u8; 2];
    match reader.read_exact(&mut is_tiff) {
        Ok(..) => {},
        Err(_) => {
            return Err(TifErrorState::InvalidHeader);
        }
    }
    println!("Is TIFF: {:?}", is_tiff);

    if !(is_tiff[0] == 42) && (is_tiff[1] == 0) && matches!(byte_order, LittleEndian) {
        return Err(InvalidHeader);
    } else if !(is_tiff[1] == 42) && (is_tiff[0] == 0) && matches!(byte_order, BigEndian) {
        return Err(InvalidHeader);
    }

    let mut offset = [0u8; 4];
    match reader.read_exact(&mut offset) {
        Ok(..) => {},
        Err(_) => {
            return Err(InvalidHeader);
        }
    }
    let offset: u32 = match byte_order {
        LittleEndian => u32::from_le_bytes(offset),
        BigEndian => u32::from_be_bytes(offset)
    };

    return Ok((byte_order, offset));
}

pub fn parse_tiff(descriptor: Descriptor) -> Result<Region, ParsingErrorState> {
    match File::open(&descriptor.path) {
        Ok(file_handle) => {
            let mut reader = BufReader::new(file_handle);
            let (byte_order, offset) = match parse_header(&mut reader) {
                Ok((byte_order, offset)) => (byte_order, offset),
                Err(_) => {
                    eprintln!("Encountered Error! Invalid Header!");
                    panic!();
                }
            };
            println!("Byte Order: {:?}, Offset: {}", byte_order, offset);


            return Ok(Region{
                bottom_left: (0.0,0.0),
                top_right: (0.0,0.0)
            })
        },
        Err(e) => Err(ParsingErrorState::FileError(descriptor, e.kind()))
    }
}