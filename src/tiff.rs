use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use crate::parsing::{Descriptor, ParsingErrorState};
use crate::spatial::Region;
use crate::tiff::ByteOrder::{BigEndian, LittleEndian};
use crate::tiff::TifErrorState::{InvalidHeader, InvalidIFD};

#[derive(Debug, Clone)]
enum ByteOrder {
    LittleEndian,
    BigEndian
}
#[derive(Debug, Clone)]
pub enum IFDValue {
    BYTE(Vec<u8>),
    ASCII(String),
    SHORT(Vec<u16>),
    LONG(Vec<u32>),
    RATIONAL(),
    UNDEFINED(Vec<u8>),
    DOUBLE(f64)
}

#[derive(Debug, Clone)]
pub enum IFDFieldType {
    BYTE,
    ASCII,
    SHORT,
    LONG,
    RATIONAL,
    UNDEFINED,
    DOUBLE,
}

impl IFDFieldType {
    fn from_integer(from: u16) -> IFDFieldType {
        return match from {
            1 => IFDFieldType::BYTE,
            2 => IFDFieldType::ASCII,
            3 => IFDFieldType::SHORT,
            4 => IFDFieldType::LONG,
            5 => IFDFieldType::RATIONAL,
            7 => IFDFieldType::UNDEFINED,
            12 => IFDFieldType::DOUBLE,
            _ => {
                eprintln!("UNHANDLED FIELD TYPE {}", from);
                panic!();
            }
        };
    }
}


#[derive(Debug, Clone)]
enum TifErrorState {
    InvalidHeader,
    InvalidIFD
}

#[derive(Debug, Clone)]
pub struct IFDEntry {
    pub tag: u16,
    pub field_type: IFDFieldType,
    pub count: u32,
    pub value: Option<IFDValue>,
    pub offset: Option<SeekFrom>
}

impl IFDEntry {
    fn new(entry_buf: &[u8; 12], byte_order: &ByteOrder) -> Result<IFDEntry, TifErrorState> {
        let tag = match byte_order {
            ByteOrder::BigEndian => u16::from_be_bytes(entry_buf[0..2].try_into().unwrap()),
            ByteOrder::LittleEndian => u16::from_le_bytes(entry_buf[0..2].try_into().unwrap())
        };

        let field_type = match byte_order {
            ByteOrder::LittleEndian => IFDFieldType::from_integer( u16::from_le_bytes(entry_buf[2..4].try_into().unwrap())),
            ByteOrder::BigEndian => IFDFieldType::from_integer(u16::from_be_bytes(entry_buf[2..4].try_into().unwrap()))
        };

        let count = match byte_order {
            ByteOrder::LittleEndian => (u32::from_le_bytes(entry_buf[4..8].try_into().unwrap())),
            ByteOrder::BigEndian => (u32::from_be_bytes(entry_buf[4..8].try_into().unwrap()))
        };

        let value_offset_bytes: [u8; 4] = entry_buf[8..12].try_into().unwrap();

        let (value, offset) = match field_type {
            IFDFieldType::BYTE => {
                if count < 5 {
                    let mut values: Vec<u8> = Vec::new();
                    for cursor in 0..count {
                        values.push(value_offset_bytes[cursor as usize]);
                    }
                    (Some(IFDValue::BYTE(values)), None)
                } else {
                    (None, Some(SeekFrom::Start(match byte_order {
                        LittleEndian => u32::from_le_bytes(value_offset_bytes),
                        BigEndian => u32::from_be_bytes(value_offset_bytes)
                    } as u64)))
                }
            },
            IFDFieldType::ASCII => {
                if count < 5 {
                    match String::from_utf8(Vec::from(value_offset_bytes)) {
                        Ok(s) => (Some(IFDValue::ASCII(s)), None),
                        Err(e) => {
                            eprintln!("Failed to parse string: {:?}", e);
                            return Err(InvalidIFD);
                        }
                    }
                } else {
                    (None, Some(SeekFrom::Start(match byte_order {
                        LittleEndian => u32::from_le_bytes(value_offset_bytes),
                        BigEndian => u32::from_be_bytes(value_offset_bytes)
                    } as u64)))
                }
            },
            IFDFieldType::SHORT => {
                if count < 3 {
                    let mut values: Vec<u16> = Vec::new();
                    for cursor in (0..count*2).step_by(2) {
                        values.push(match byte_order {
                            LittleEndian => u16::from_le_bytes([value_offset_bytes[cursor as usize], value_offset_bytes[(cursor as usize) + 1]]),
                            BigEndian => u16::from_be_bytes([value_offset_bytes[cursor as usize], value_offset_bytes[(cursor as usize) + 1]]),
                        })
                    }
                    (Some(IFDValue::SHORT(values)), None)
                } else {
                    (None, Some(SeekFrom::Start(match byte_order {
                        LittleEndian => u32::from_le_bytes(value_offset_bytes),
                        BigEndian => u32::from_be_bytes(value_offset_bytes)
                    } as u64)))
                }
            },
            IFDFieldType::LONG => {
                if count == 1 {
                    let value = vec![match byte_order {
                        LittleEndian => u32::from_le_bytes(value_offset_bytes),
                        BigEndian => u32::from_be_bytes(value_offset_bytes)
                    }];
                    (Some(IFDValue::LONG(value)), None)
                } else {
                    (None, Some(SeekFrom::Start(match byte_order {
                        LittleEndian => u32::from_le_bytes(value_offset_bytes),
                        BigEndian => u32::from_be_bytes(value_offset_bytes)
                    } as u64)))
                }
            },
            IFDFieldType::RATIONAL => {
                (None, Some(SeekFrom::Start(match byte_order {
                    LittleEndian => u32::from_le_bytes(value_offset_bytes),
                    BigEndian => u32::from_be_bytes(value_offset_bytes)
                } as u64)))
            }
            IFDFieldType::UNDEFINED => {
                if count < 5 {
                    let mut values: Vec<u8> = Vec::new();
                    for cursor in 0..count {
                        values.push(value_offset_bytes[cursor as usize]);
                    }
                    (Some(IFDValue::UNDEFINED(values)), None)
                } else {
                    (None, Some(SeekFrom::Start(match byte_order {
                        LittleEndian => u32::from_le_bytes(value_offset_bytes),
                        BigEndian => u32::from_be_bytes(value_offset_bytes)
                    } as u64)))
                }
            },
            IFDFieldType::DOUBLE => {
                (None, Some(SeekFrom::Start(match byte_order {
                    LittleEndian => u32::from_le_bytes(value_offset_bytes),
                    BigEndian => u32::from_be_bytes(value_offset_bytes)
                } as u64)))
            }
        };

        return Ok(IFDEntry {
            tag,
            field_type,
            count,
            value,
            offset
        })
    }
    fn resolve_value(&self, reader: &mut BufReader<File>) -> Result<&IFDValue, TifErrorState> {
        todo!()
    }
}



fn parse_header(reader: &mut BufReader<File>) -> Result<(ByteOrder, u32), TifErrorState> {
    match reader.seek(SeekFrom::Start(0)) {
        Ok(..) => {
            let mut buf = [0u8; 8];
            let byte_order = match reader.read_exact(&mut buf) {
                Ok(..) => {
                    if buf[0..2] == [73, 73] {
                        LittleEndian
                    } else if buf[0..2] == [77, 77] {
                        BigEndian
                    } else {
                        return Err(InvalidHeader);
                    }
                },
                Err(_) => {
                    return Err(InvalidHeader);
                }
            };
            if !match &byte_order {
                LittleEndian => (buf[2..4][0] == 42) && (buf[2..4][1] == 0),
                BigEndian => (buf[2..4][0] == 0) && (buf[2..4][1] == 42)
            } {
                // Is not tiff!
                return Err(InvalidHeader);
            }
            return Ok((byte_order.clone(), match &byte_order {
                LittleEndian => u32::from_le_bytes(buf[4..8].try_into().unwrap()),
                BigEndian => u32::from_be_bytes(buf[4..8].try_into().unwrap())
            }));
        },
        Err(_) => Err(InvalidHeader)
    }
}


fn parse_ifd(reader: &mut BufReader<File>, offset: SeekFrom, byte_order: &ByteOrder) -> Result<(Vec<IFDEntry>, Option<SeekFrom>), TifErrorState> {
    return match reader.seek(offset) {
        Ok(..) => {
            let mut entry_count_buf = [0u8; 2];
            let entry_count = match reader.read_exact(&mut entry_count_buf) {
                Ok(..) => match byte_order {
                    BigEndian => u16::from_be_bytes(entry_count_buf),
                    LittleEndian => u16::from_le_bytes(entry_count_buf)
                },
                Err(_) => {
                    return Err(InvalidIFD);
                }
            };
            let mut entry_buf = [0u8; 12];
            let mut entries: Vec<IFDEntry> = Vec::with_capacity(entry_count as usize);
            for entry_number in 0..entry_count {
                println!("Reading IFD Entry: #{}", entry_number);
                match reader.read_exact(&mut entry_buf) {
                    Ok(..) => {
                        entries.push(match IFDEntry::new(&entry_buf, &byte_order) {
                            Ok(e) => e,
                            Err(_) => {
                                return Err(InvalidIFD);
                            }
                        });
                    },
                    Err(_) => {
                        return Err(InvalidIFD);
                    }
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
                    return Err(InvalidIFD);
                }
            };
            Ok((entries, Some(SeekFrom::Start(next_ifd as u64))))
        }
        Err(_) => {
            Err(InvalidIFD)
        }
    }
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
            let (entries, next_ifd) = match parse_ifd(&mut reader, SeekFrom::Start(offset as u64), &byte_order) {
                Ok(x) => x,
                Err(_) => {
                    eprintln!("Encountered Error! Invalid IFD!");
                    panic!();
                }
            };
            println!("Entries: {:?}", entries);
            println!("Next IFD: {:?}", next_ifd);

            return Ok(Region{
                bottom_left: (0.0,0.0),
                top_right: (0.0,0.0)
            })
        },
        Err(e) => Err(ParsingErrorState::FileError(descriptor, e.kind()))
    }
}