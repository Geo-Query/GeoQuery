use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use crate::err::TIFFErrorState;
use crate::geokeydirectory::GeoKeyDirectory;
use crate::geokeydirectory::GeoKeyDirectoryError::UnexpectedFormat;
use crate::tag::{EntryValue, IFDEntry};

// ByteOrder enum.
#[derive(Debug, Clone)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian
}

// Integer parsing impls.
// Implemented for u16, u32, and f64
pub trait FromBytes {
    fn from_bytes(bytes: &[u8], byte_order: &ByteOrder) -> Self;
}

impl FromBytes for u16 {
    fn from_bytes(bytes: &[u8], byte_order: &ByteOrder) -> Self {
        if bytes.len() != 2 { panic!("Unexpected number of bytes passed!"); }
        let bytes = bytes.try_into().unwrap();
        return match byte_order {
            ByteOrder::LittleEndian => u16::from_le_bytes(bytes),
            ByteOrder::BigEndian => u16::from_be_bytes(bytes)
        }
    }
}

impl FromBytes for u32 {
    fn from_bytes(bytes: &[u8], byte_order: &ByteOrder) -> Self {
        if bytes.len() != 4 { panic!("Unexpected number of bytes passed!"); }
        let bytes = bytes.try_into().unwrap();
        return match byte_order {
            ByteOrder::LittleEndian => u32::from_le_bytes(bytes),
            ByteOrder::BigEndian => u32::from_be_bytes(bytes)
        }
    }
}
impl FromBytes for f64 {
    fn from_bytes(bytes: &[u8], byte_order: &ByteOrder) -> Self {
        if bytes.len() != 8 { panic!("Unexpected number of bytes passed!"); }
        let bytes = bytes.try_into().unwrap();
        return match byte_order {
            ByteOrder::LittleEndian => f64::from_le_bytes(bytes),
            ByteOrder::BigEndian => f64::from_be_bytes(bytes)
        }
    }
}


