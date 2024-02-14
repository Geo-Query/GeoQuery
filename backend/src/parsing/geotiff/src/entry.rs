use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use util::ByteOrder;
use crate::util;
use crate::error::{IFDEntryErrorState, TIFFErrorState};
use crate::util::FromBytes;


#[derive(Debug, Clone, PartialEq)]
pub enum EntryType {
    BYTES,
    ASCII,
    SHORT,
    LONG,
    RATIONAL,
    UNDEFINED,
    DOUBLE
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntryValue {
    BYTES(Vec<u8>),
    ASCII(Vec<String>),
    SHORT(Vec<u16>),
    LONG(Vec<u32>),
    RATIONAL(Vec<(u32, u32)>),
    UNDEFINED(Vec<u8>),
    DOUBLE(Vec<f64>)
}

#[derive(Debug, Clone)]
pub struct IFDEntry {
    pub(crate) tag: u16,
    count: u32,
    field_type: EntryType,
    associated_bytes: [u8; 4],
    value: Option<EntryValue>
}


impl IFDEntry {
    pub fn new(entry_buf: &[u8], byte_order: &ByteOrder) -> Result<IFDEntry, TIFFErrorState> {
        if entry_buf.len() != 12 { // Ensure expected entry buffer length (Entries are 12 bytes)
            return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::InvalidLength(entry_buf.len())));
        }

        let tag = u16::from_bytes(&entry_buf[0..2], &byte_order); // Get tag ID

        // Get field type, throw an error if unhandled.
        let field_type = u16::from_bytes(&entry_buf[2..4], &byte_order);
        let field_type = match field_type {
            1 => EntryType::BYTES,
            2 => EntryType::ASCII,
            3 => EntryType::SHORT,
            4 => EntryType::LONG,
            5 => EntryType::RATIONAL,
            7 => EntryType::UNDEFINED,
            12 => EntryType::DOUBLE,
            _ => return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::UnexpectedEntryType(field_type)))
        };

        // Get count of values
        let count = u32::from_bytes(&entry_buf[4..8], &byte_order);
        let associated_bytes: [u8; 4] = [entry_buf[8], entry_buf[9], entry_buf[10], entry_buf[11]];
        return Ok(IFDEntry {
            tag,
            count,
            field_type,
            associated_bytes,
            value: None,
        });
    }



    pub fn resolve(&mut self, byte_order: &ByteOrder, reader: &mut BufReader<File>) -> Result<&EntryValue, TIFFErrorState> {
        if self.value.is_none() {
            let value: EntryValue = match &self.field_type {
                EntryType::BYTES => EntryValue::BYTES(if self.count < 5 {
                    Vec::from(self.associated_bytes)
                } else {
                    let offset = SeekFrom::Start(u32::from_bytes(&self.associated_bytes, &byte_order) as u64);
                    match reader.seek(offset) {
                        Ok(_) => {
                            let mut values = vec![0u8; self.count as usize];
                            match reader.read_exact(&mut values) {
                                Ok(..) => {
                                    values
                                },
                                Err(e) => {
                                    eprintln!("Failed to read bytes for tag: {}, Error: {:?}", self.tag, e);
                                    return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)))
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to seek to associated bytes for tag: {}, Error: {:?}", self.tag, e);
                            return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)))
                        }
                    }
                }),
                EntryType::ASCII => EntryValue::ASCII(if self.count < 5 {
                    match String::from_utf8(self.associated_bytes.to_vec()) {
                        Ok(s) => vec![s],
                        Err(e) => {
                            eprintln!("Failed to parse string from bytes for tag: {}, Error: {:?}", self.tag, e);
                            return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)));
                        }
                    }
                } else {
                    let offset = SeekFrom::Start(u32::from_bytes(&self.associated_bytes, &byte_order) as u64);

                    match reader.seek(offset) {
                        Ok(_) => {
                            let mut bytes = vec![0u8; self.count as usize];

                            match reader.read_exact(&mut bytes) {
                                Ok(..) => match String::from_utf8(bytes) {
                                    Ok(s) => vec![s],
                                    Err(e) => {
                                        eprintln!("Failed to parse string from bytes for tag: {}, Error: {:?}", self.tag, e);
                                        return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)));
                                    }
                                },
                                Err(e) => {
                                    eprintln!("Failed to read bytes for tag: {}, Error: {:?}", self.tag, e);
                                    return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)))
                                }
                            }
                        },
                        Err(e) => {
                                eprintln!("Failed to seek to associated bytes for tag: {}, Error: {:?}", self.tag, e);
                                return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)))
                        }
                    }
                }),
                EntryType::SHORT => {
                    let mut values: Vec<u16> = Vec::with_capacity(self.count as usize);

                    if self.count < 3 {
                        for cursor in (0..self.count*2).step_by(2) {
                            values.push(u16::from_bytes(&self.associated_bytes[(cursor as usize)..(cursor as usize)+2], &byte_order));
                        }
                        EntryValue::SHORT(values)
                    } else {
                        let offset = SeekFrom::Start(u32::from_bytes(&self.associated_bytes, &byte_order) as u64);

                        match reader.seek(offset) {
                            Ok(_) => {
                                for _ in 0..self.count {
                                    let mut value_buf = [0u8, 0u8];
                                    match reader.read_exact(&mut value_buf) {
                                        Ok(..) => {
                                            values.push(u16::from_bytes(&value_buf, &byte_order))
                                        },
                                        Err(e) => {
                                            eprintln!("Failed to read bytes for tag: {}, Error: {:?}", self.tag, e);
                                            return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)))                                        }
                                    }
                                }
                                EntryValue::SHORT(values)
                            },
                            Err(e) => {
                                eprintln!("Failed to seek to associated bytes for tag: {}, Error: {:?}", self.tag, e);
                                return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)))
                            }
                        }
                    }
                },
                EntryType::LONG => {
                    let mut values: Vec<u32> = Vec::with_capacity((self.count) as usize);

                    if self.count == 1 {
                        values.push(u32::from_bytes(&self.associated_bytes, &byte_order));
                        EntryValue::LONG(values)
                    } else {
                        let offset = SeekFrom::Start(u32::from_bytes(&self.associated_bytes, &byte_order) as u64);

                        match reader.seek(offset) {
                            Ok(_) => {
                                for _ in 0..self.count {
                                    let mut value_buf = [0u8; 4];
                                    match reader.read_exact(&mut value_buf) {
                                        Ok(..) => {
                                            values.push(u32::from_bytes(&value_buf, byte_order));
                                        },
                                        Err(e) => {
                                            eprintln!("Failed to read bytes for tag: {}, Error: {:?}", self.tag, e);
                                            return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)))                                                    }
                                    }
                                }
                                EntryValue::LONG(values)
                            },
                            Err(e) => {
                                eprintln!("Failed to seek to associated bytes for tag: {}, Error: {:?}", self.tag, e);
                                return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)))
                            }
                        }
                    }
                },
                EntryType::RATIONAL => {
                    let offset = SeekFrom::Start(u32::from_bytes(&self.associated_bytes, &byte_order) as u64);
                    match reader.seek(offset) {
                        Ok(_) => {
                            let mut values: Vec<(u32, u32)> = Vec::with_capacity(self.count as usize);
                            for _ in 0..self.count {
                                let mut value_buf = [0u8; 8];
                                match reader.read_exact(&mut value_buf) {
                                    Ok(..) => {
                                        values.push((
                                            u32::from_bytes(&value_buf[0..4], &byte_order),
                                            u32::from_bytes(&value_buf[4..8], &byte_order),
                                        ));
                                    },
                                    Err(e) => {
                                        eprintln!("Failed to read bytes for tag: {}, Error: {:?}", self.tag, e);
                                        return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)))
                                    }
                                }
                            };
                            EntryValue::RATIONAL(values)

                        },
                        Err(e) => {
                            eprintln!("Failed to seek to associated bytes for tag: {}, Error: {:?}", self.tag, e);
                            return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)))
                        }
                    }
                }
                EntryType::DOUBLE => {
                    let mut values: Vec<f64> = Vec::with_capacity(self.count as usize);
                    let offset = SeekFrom::Start(u32::from_bytes(&self.associated_bytes, &byte_order) as u64);


                    match reader.seek(offset) {
                        Ok(_) => {
                            for _ in 0..self.count {
                                let mut value_buf = [0u8; 8];
                                match reader.read_exact(&mut value_buf) {
                                    Ok(..) => {
                                        values.push(f64::from_bytes(&value_buf, &byte_order));
                                    },
                                    Err(e) => {
                                        eprintln!("Failed to read bytes for tag: {}, Error: {:?}", self.tag, e);
                                        return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)))
                                    }
                                }
                            };
                            EntryValue::DOUBLE(values)

                        },
                        Err(e) => {
                            eprintln!("Failed to seek to associated bytes for tag: {}, Error: {:?}", self.tag, e);
                            return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)))
                        }
                    }
                },
                EntryType::UNDEFINED => EntryValue::UNDEFINED(if self.count < 5 {
                    Vec::from(self.associated_bytes)
                } else {
                    let offset = SeekFrom::Start(u32::from_bytes(&self.associated_bytes, &byte_order) as u64);
                    match reader.seek(offset) {
                        Ok(_) => {
                            let mut bytes = vec![0u8; self.count as usize];
                            match reader.read_exact(&mut bytes) {
                                Ok(..) => {
                                    bytes
                                },
                                Err(e) => {
                                    eprintln!("Failed to read bytes for tag: {}, Error: {:?}", self.tag, e);
                                    return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)))
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to seek to associated bytes for tag: {}, Error: {:?}", self.tag, e);
                            return Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::MissingAssociatedValue(self.tag)))
                        }
                    }
                })
            };

            self.value = Some(value);
            return Ok(self.value.as_ref().unwrap());
        } else {
            return Ok(self.value.as_ref().unwrap());
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::ByteOrder;
    use tempfile::tempfile;
    #[test]
    fn test_ifd_entry_new() {
        // Create expected IFD entry byte sequence
        let entry_buf: [u8; 12] = [0, 1, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0];
        let byte_order = ByteOrder::BigEndian;

        // Attempt to parse IFDEntry
        let result = IFDEntry::new(&entry_buf, &byte_order);

        assert!(result.is_ok(), "Failed to parse IFDEntry: {:?}", result);

        let ifd_entry = result.expect("Failed to parse IFDEntry");

        // Verify if each field is correctly parsed
        assert_eq!(ifd_entry.tag, 1);
        assert_eq!(ifd_entry.field_type, EntryType::SHORT);
        assert_eq!(ifd_entry.count, 1);
        assert_eq!(ifd_entry.associated_bytes, [0, 0, 0, 0]);
    }
    #[test]
    fn test_resolve_bytes() {
        let mut file = tempfile().unwrap();
        file.write_all(&[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        let mut reader = BufReader::new(file);

        let mut entry = IFDEntry {
            tag: 0,
            count: 4,
            field_type: EntryType::BYTES,
            associated_bytes: [0, 1, 2, 3],
            value: None,
        };

        let result = entry.resolve(&ByteOrder::LittleEndian, &mut reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), &EntryValue::BYTES(vec![0, 1, 2, 3]));
    }

    #[test]
    fn test_resolve_ascii() {
        let mut file = tempfile().unwrap();
        file.write_all(b"Hello\0").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        let mut reader = BufReader::new(file);

        let mut entry = IFDEntry {
            tag: 0,
            count: 6,
            field_type: EntryType::ASCII,
            associated_bytes: [0; 4], // Assume offset is at the start of the file
            value: None,
        };

        let result = entry.resolve(&ByteOrder::LittleEndian, &mut reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), &EntryValue::ASCII(vec!["Hello".to_string()]));
    }

    #[test]
    fn test_resolve_short_direct() {
        let byte_order = ByteOrder::LittleEndian;
        let mut entry = IFDEntry {
            tag: 0,
            count: 2,
            field_type: EntryType::SHORT,
            associated_bytes: [1, 0, 2, 0], // Two SHORT values: 1, 2
            value: None,
        };

        let file = tempfile().expect("Failed to create tempfile");
        let mut reader = BufReader::new(file);
        let result = entry.resolve(&byte_order, &mut reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), &EntryValue::SHORT(vec![1, 2]));
    }

    #[test]
    fn test_resolve_ascii_offset() {
        let byte_order = ByteOrder::LittleEndian;
        let text = "Hello\0";
        let mut file = tempfile().expect("Failed to create tempfile");
        file.write_all(text.as_bytes()).expect("Failed to write to tempfile");
        file.seek(SeekFrom::Start(0)).expect("Failed to seek to start of tempfile");
        let mut reader = BufReader::new(file);

        let mut entry = IFDEntry {
            tag: 0,
            count: text.len() as u32,
            field_type: EntryType::ASCII,
            associated_bytes: [0, 0, 0, 0], // Offset to the ASCII value
            value: None,
        };

        let result = entry.resolve(&byte_order, &mut reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), &EntryValue::ASCII(vec!["Hello".to_string()]));
    }

    #[test]
    fn test_resolve_long() {
        let mut file = tempfile().unwrap();
        file.write_all(&[1, 0, 0, 0, 2, 0, 0, 0]).unwrap(); // Two LONG value：1, 2
        file.seek(SeekFrom::Start(0)).unwrap();
        let mut reader = BufReader::new(file);

        let mut entry = IFDEntry {
            tag: 0,
            count: 2,
            field_type: EntryType::LONG,
            associated_bytes: [0, 0, 0, 0],
            value: None,
        };

        let result = entry.resolve(&ByteOrder::LittleEndian, &mut reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), &EntryValue::LONG(vec![1, 2]));
    }

    #[test]
    fn test_resolve_rational() {
        let mut file = tempfile().unwrap();
        file.write_all(&[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]).unwrap(); // Two RATIONAL value：(1, 2), (3, 4)
        file.seek(SeekFrom::Start(0)).unwrap();
        let mut reader = BufReader::new(file);

        let mut entry = IFDEntry {
            tag: 0,
            count: 2,
            field_type: EntryType::RATIONAL,
            associated_bytes: [0, 0, 0, 0],
            value: None,
        };

        let result = entry.resolve(&ByteOrder::LittleEndian, &mut reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), &EntryValue::RATIONAL(vec![(1, 2), (3, 4)]));
    }

    #[test]
    fn test_invalid_entry_length() {
        let entry_buf = [0u8; 11]; // Invalid entry length, should be 12
        let byte_order = ByteOrder::LittleEndian;

        let result = IFDEntry::new(&entry_buf, &byte_order);
        assert!(matches!(result, Err(TIFFErrorState::IFDEntryError(IFDEntryErrorState::InvalidLength(_)))));
    }

    #[test]
    fn test_empty_ascii_entry() {
        let mut entry = IFDEntry {
            tag: 0,
            count: 0, // ASCII string length is 0
            field_type: EntryType::ASCII,
            associated_bytes: [0; 4],
            value: None,
        };

        let file = tempfile().expect("Failed to create tempfile");
        let mut reader = BufReader::new(file);

        let result = entry.resolve(&ByteOrder::LittleEndian, &mut reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), &EntryValue::ASCII(vec!["".to_string()]));
    }




}


