use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use util::ByteOrder;
use crate::err::TIFFErrorState;
use crate::util;
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

impl IFDEntry {
    pub(crate) fn new(entry_buf: [u8; 12], byte_order: &ByteOrder) -> Result<IFDEntry, TIFFErrorState>{
        let tag = u16::from_bytes(&entry_buf[0..2], &byte_order);
        let field_type = match u16::from_bytes(&entry_buf[2..4], &byte_order) {
            1 => EntryType::BYTES,
            2 => EntryType::ASCII,
            3 => EntryType::SHORT,
            4 => EntryType::LONG,
            5 => EntryType::RATIONAL,
            7 => EntryType::UNDEFINED,
            12 => EntryType::DOUBLE,
            e => {
                eprintln!("{}",  e);
                return Err(TIFFErrorState::UnexpectedFormat("Unhandled tag type".to_string()));
            }
        };
        let count = u32::from_bytes(&entry_buf[4..8], &byte_order);
        let associated_bytes: [u8; 4] = entry_buf[8..12].try_into().unwrap();
        return match field_type {
            EntryType::BYTES => Ok(IFDEntry {
                tag,
                count,
                field_type,
                associated_bytes,
                value: None,
            }),
            EntryType::ASCII => Ok(IFDEntry {
                tag,
                count,
                field_type,
                associated_bytes,
                value: None,
            }),
            EntryType::SHORT => Ok(IFDEntry {
                tag,
                count,
                field_type,
                associated_bytes,
                value: None,
            }),
            EntryType::LONG => Ok(IFDEntry {
                tag,
                count,
                field_type,
                associated_bytes,
                value: None,
            }),
            EntryType::RATIONAL => Ok(IFDEntry {
                tag,
                count,
                field_type,
                associated_bytes,
                value: None,
            }),
            EntryType::UNDEFINED => Ok(IFDEntry {
                tag,
                count,
                field_type,
                associated_bytes,
                value: None,
            }),
            EntryType::DOUBLE => Ok(IFDEntry {
                tag,
                count,
                field_type,
                associated_bytes,
                value: None,
            })
        }
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
                            let mut value = Vec::with_capacity(self.count as usize);

                            for _ in 0..self.count {
                                let mut value_buf = [0u8];
                                match reader.read_exact(&mut value_buf) {
                                    Ok(..) => {
                                        value.push(value_buf[0]);
                                    },
                                    Err(_) => {
                                        return Err(TIFFErrorState::FailedToParseTag)
                                    }
                                }
                            }
                            value
                        },
                        Err(_) => {
                            return Err(TIFFErrorState::FailedToParseTag);
                        }
                    }
                }),
                EntryType::ASCII => EntryValue::ASCII(if self.count < 5 {
                    match String::from_utf8(self.associated_bytes.to_vec()) {
                        Ok(s) => vec![s],
                        Err(_) => {
                            return Err(TIFFErrorState::FailedToParseTag);
                        }
                    }
                } else {
                    let offset = SeekFrom::Start(u32::from_bytes(&self.associated_bytes, &byte_order) as u64);

                    match reader.seek(offset) {
                        Ok(_) => {
                            let mut bytes: Vec<u8> = Vec::with_capacity(self.count as usize);

                            for _ in 0..self.count {
                                let mut value_buf = [0u8];

                                match reader.read_exact(&mut value_buf) {
                                    Ok(..) => {
                                        bytes.push(value_buf[0]);
                                    },
                                    Err(_) => {
                                        return Err(TIFFErrorState::FailedToParseTag);
                                    }
                                }
                            }

                            match String::from_utf8(bytes) {
                                Ok(s) => vec![s],
                                Err(_) => {
                                    return Err(TIFFErrorState::FailedToParseTag);
                                }
                            }
                        },
                        Err(_) => {
                            return Err(TIFFErrorState::FailedToParseTag);
                        }
                    }
                }),
                EntryType::SHORT => {
                    let mut value: Vec<u16> = Vec::with_capacity(self.count as usize);

                    if self.count < 3 {
                        for cursor in (0..self.count*2).step_by(2) {
                            value.push(u16::from_bytes(&[self.associated_bytes[cursor as usize], self.associated_bytes[cursor as usize +1]], &byte_order));
                        }
                        EntryValue::SHORT(value)
                    } else {
                        let offset = SeekFrom::Start(u32::from_bytes(&self.associated_bytes, &byte_order) as u64);

                        match reader.seek(offset) {
                            Ok(_) => {
                                for _ in 0..self.count {
                                    let mut value_buf = [0u8, 0u8];
                                    match reader.read_exact(&mut value_buf) {
                                        Ok(..) => {
                                            value.push(u16::from_bytes(&value_buf, &byte_order))
                                        },
                                        Err(_) => {
                                            return Err(TIFFErrorState::FailedToParseTag)
                                        }
                                    }
                                }
                                EntryValue::SHORT(value)
                            },
                            Err(_) => {
                                return Err(TIFFErrorState::FailedToParseTag);
                            }
                        }
                    }
                },
                EntryType::LONG => {
                    let mut value: Vec<u32> = Vec::with_capacity((self.count) as usize);

                    if self.count == 1 {
                        value.push(u32::from_bytes(&self.associated_bytes, &byte_order));
                        EntryValue::LONG(value)
                    } else {
                        let offset = SeekFrom::Start(u32::from_bytes(&self.associated_bytes, &byte_order) as u64);

                        match reader.seek(offset) {
                            Ok(_) => {
                                for _ in 0..self.count {
                                    let mut value_buf = [0u8; 4];
                                    match reader.read_exact(&mut value_buf) {
                                        Ok(..) => {
                                            value.push(u32::from_bytes(&value_buf, byte_order));
                                        },
                                        Err(_) => {
                                            return Err(TIFFErrorState::FailedToParseTag);
                                        }
                                    }
                                }
                                EntryValue::LONG(value)
                            },
                            Err(_) => {
                                return Err(TIFFErrorState::FailedToParseTag);
                            }
                        }
                    }
                },
                EntryType::RATIONAL => {
                    let offset = SeekFrom::Start(u32::from_bytes(&self.associated_bytes, &byte_order) as u64);
                    match reader.seek(offset) {
                        Ok(_) => {
                            let mut value: Vec<(u32, u32)> = Vec::with_capacity(self.count as usize);
                            for _ in 0..self.count {
                                let mut value_buf = [0u8; 8];
                                match reader.read_exact(&mut value_buf) {
                                    Ok(..) => {
                                        value.push((
                                            u32::from_bytes(&value_buf[0..4], &byte_order),
                                            u32::from_bytes(&value_buf[4..8], &byte_order),
                                        ));
                                    },
                                    Err(_) => {
                                        return Err(TIFFErrorState::FailedToParseTag);
                                    }
                                }
                            };
                            EntryValue::RATIONAL(value)

                        },
                        Err(_) => {
                            return Err(TIFFErrorState::FailedToParseTag);
                        }
                    }
                }
                EntryType::DOUBLE => {
                    let mut value: Vec<f64> = Vec::with_capacity(self.count as usize);
                    let offset = SeekFrom::Start(u32::from_bytes(&self.associated_bytes, &byte_order) as u64);


                    match reader.seek(offset) {
                        Ok(_) => {
                            for _ in 0..self.count {
                                let mut value_buf = [0u8; 8];
                                match reader.read_exact(&mut value_buf) {
                                    Ok(..) => {
                                        value.push(f64::from_bytes(&value_buf, &byte_order));
                                    },
                                    Err(_) => {
                                        return Err(TIFFErrorState::FailedToParseTag);
                                    }
                                }
                            };
                            EntryValue::DOUBLE(value)

                        },
                        Err(_) => {
                            return Err(TIFFErrorState::FailedToParseTag);
                        }
                    }
                },
                EntryType::UNDEFINED => EntryValue::UNDEFINED(if self.count < 5 {
                    Vec::from(self.associated_bytes)
                } else {
                    let offset = SeekFrom::Start(u32::from_bytes(&self.associated_bytes, &byte_order) as u64);
                    match reader.seek(offset) {
                        Ok(_) => {
                            let mut value = Vec::with_capacity(self.count as usize);

                            for _ in 0..self.count {
                                let mut value_buf = [0u8];
                                match reader.read_exact(&mut value_buf) {
                                    Ok(..) => {
                                        value.push(value_buf[0]);
                                    },
                                    Err(_) => {
                                        return Err(TIFFErrorState::FailedToParseTag);
                                    }
                                }
                            }
                            value
                        },
                        Err(_) => {
                            return Err(TIFFErrorState::FailedToParseTag);
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