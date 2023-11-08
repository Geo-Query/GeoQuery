use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use crate::err::TIFFErrorState;
use crate::tag::IFDEntry;
use crate::util::ByteOrder;


pub(crate) fn parse_ifd(reader: &mut BufReader<File>, offset: SeekFrom, byte_order: &ByteOrder) -> Result<(Vec<IFDEntry>, Option<SeekFrom>), TIFFErrorState> {
    return match reader.seek(offset) {
        Ok(..) => {
            let mut entry_count_buf = [0u8; 2];
            let entry_count = match reader.read_exact(&mut entry_count_buf) {
                Ok(..) => match byte_order {
                    ByteOrder::BigEndian => u16::from_be_bytes(entry_count_buf),
                    ByteOrder::LittleEndian => u16::from_le_bytes(entry_count_buf)
                },
                Err(_) => {
                    return Err(TIFFErrorState::UnexpectedFormat("Unexpected Header Format.".to_string()));
                }
            };
            let mut entry_buf = [0u8; 12];
            let mut entries: Vec<IFDEntry> = Vec::with_capacity(entry_count as usize);
            for entry_number in 0..entry_count {
                println!("Reading IFD Entry: #{}", entry_number);
                match reader.read_exact(&mut entry_buf) {
                    Ok(..) => {
                        entries.push(match IFDEntry::new(entry_buf, &byte_order) {
                            Ok(e) => e,
                            Err(e) => {
                                eprintln!("{:?}", e);
                                return Err(TIFFErrorState::UnexpectedFormat("Unexpected IFD Format.".to_string()));                            }
                        });
                    },
                    Err(_) => {
                        return Err(TIFFErrorState::UnexpectedFormat("Unexpected Header Format.".to_string()));                    }
                }
            }
            let mut next_ifd_buf = [0u8; 4];
            let next_ifd = match reader.read_exact(&mut next_ifd_buf) {
                Ok(..) => {
                    match byte_order {
                        ByteOrder::LittleEndian => u32::from_le_bytes(next_ifd_buf),
                        ByteOrder::BigEndian => u32::from_be_bytes(next_ifd_buf)
                    }
                },
                Err(_) => {
                    return Err(TIFFErrorState::UnexpectedFormat("Unexpected IFD Format.".to_string()));                }
            };
            Ok((entries, Some(SeekFrom::Start(next_ifd as u64))))
        }
        Err(_) => {
            return Err(TIFFErrorState::UnexpectedFormat("Unexpected Header Format.".to_string()));        }
    }
}