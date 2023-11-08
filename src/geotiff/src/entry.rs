use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use util::ByteOrder;
use crate::{TIFFErrorState, util};
use crate::entry::IFDEntryErrorState::MissingAssociatedValue;
use crate::util::FromBytes;


#[derive(Debug, Clone)]
pub enum EntryType {
    BYTES,
    ASCII,
    SHORT,
    LONG,
    RATIONAL,
    UNDEFINED,
    DOUBLE
}

#[derive(Debug, Clone)]
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

pub enum IFDEntryErrorState {
    UnexpectedEntryType(u16),
    MissingAssociatedValue(u16),
    InvalidLength(usize)
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



    pub(crate) fn resolve(&mut self, byte_order: &ByteOrder, reader: &mut BufReader<File>) -> Result<&EntryValue, TIFFErrorState> {
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
                                    return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)))
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to seek to associated bytes for tag: {}, Error: {:?}", self.tag, e);
                            return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)))
                        }
                    }
                }),
                EntryType::ASCII => EntryValue::ASCII(if self.count < 5 {
                    match String::from_utf8(self.associated_bytes.to_vec()) {
                        Ok(s) => vec![s],
                        Err(e) => {
                            eprintln!("Failed to parse string from bytes for tag: {}, Error: {:?}", self.tag, e);
                            return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)));
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
                                        return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)));
                                    }
                                },
                                Err(e) => {
                                    eprintln!("Failed to read bytes for tag: {}, Error: {:?}", self.tag, e);
                                    return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)))
                                }
                            }
                        },
                        Err(e) => {
                                eprintln!("Failed to seek to associated bytes for tag: {}, Error: {:?}", self.tag, e);
                                return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)))
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
                                            return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)))                                        }
                                    }
                                }
                                EntryValue::SHORT(values)
                            },
                            Err(e) => {
                                eprintln!("Failed to seek to associated bytes for tag: {}, Error: {:?}", self.tag, e);
                                return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)))
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
                                            return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)))                                                    }
                                    }
                                }
                                EntryValue::LONG(values)
                            },
                            Err(e) => {
                                eprintln!("Failed to seek to associated bytes for tag: {}, Error: {:?}", self.tag, e);
                                return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)))
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
                                        return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)))
                                    }
                                }
                            };
                            EntryValue::RATIONAL(values)

                        },
                        Err(e) => {
                            eprintln!("Failed to seek to associated bytes for tag: {}, Error: {:?}", self.tag, e);
                            return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)))
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
                                        return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)))
                                    }
                                }
                            };
                            EntryValue::DOUBLE(values)

                        },
                        Err(e) => {
                            eprintln!("Failed to seek to associated bytes for tag: {}, Error: {:?}", self.tag, e);
                            return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)))
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
                                    return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)))
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to seek to associated bytes for tag: {}, Error: {:?}", self.tag, e);
                            return Err(TIFFErrorState::IFDEntryError(MissingAssociatedValue(self.tag)))
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