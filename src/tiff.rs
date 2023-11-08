use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::process::exit;
use proj::Proj;
use crate::spatial::Region;
use crate::parsing::ParsingErrorState::{
    InvalidOrUnhandledFormat,
    NoGeoData,
};
use crate::parsing::{Descriptor, ParsingErrorState};

trait FromBytes {
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



#[derive(Debug, Clone)]
pub enum TIFFErrorState {
    UnexpectedFormat(String),
    MissingTag,
    FailedToParseTag,
}

#[derive(Debug, Clone)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian
}

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
    tag: u16,
    count: u32,
    field_type: EntryType,
    associated_bytes: [u8; 4],
    value: Option<EntryValue>
}

impl IFDEntry {
    fn new(entry_buf: [u8; 12], byte_order: &ByteOrder) -> Result<IFDEntry, TIFFErrorState>{
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

    fn resolve(&mut self, byte_order: &ByteOrder, reader: &mut BufReader<File>) -> Result<&EntryValue, TIFFErrorState> {
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

fn parse_header(reader: &mut BufReader<File>) -> Result<(ByteOrder, u32), TIFFErrorState> {
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

fn parse_ifd(reader: &mut BufReader<File>, offset: SeekFrom, byte_order: &ByteOrder) -> Result<(Vec<IFDEntry>, Option<SeekFrom>), TIFFErrorState> {
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

pub fn parse_tiff(descriptor: Descriptor) -> Result<Region, ParsingErrorState> {
    match File::open(&descriptor.path) {
        Ok(file_handle) => {
            let mut reader = BufReader::new(file_handle);
            let (byte_order, offset) = match parse_header(&mut reader) {
                Ok((byte_order, offset)) => (byte_order, offset),
                Err(e) => {
                    eprintln!("Encountered Error! {:?} Invalid Header!", e);
                    panic!();
                }
            };
            println!("Byte Order: {:?}, Offset: {}", byte_order, offset);
            let (mut entries, next_ifd) = match parse_ifd(&mut reader, SeekFrom::Start(offset as u64), &byte_order) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("Encountered Error! {:?} Invalid IFD!", e);
                    panic!();
                }
            };
            println!("Entries: {:?}", entries);
            let mut tag_lookup: HashMap<u16, IFDEntry> = entries.into_iter().map(|entry| (entry.tag, entry)).collect();
            println!("NEXTIFD: {:?}", next_ifd);
            println!("ImageWidth: {:?}", tag_lookup.get_mut(&256u16).expect("No ImageWidth Entry!").resolve(&byte_order, &mut reader));
            println!("ImageLength: {:?}", tag_lookup.get_mut(&257u16).expect("No ImageLength Entry!").resolve(&byte_order, &mut reader));
            println!("ModelTiePoint: {:?}", tag_lookup.get_mut(&33922u16).expect("No ModelTiePoint Entry!").resolve(&byte_order, &mut reader));
            println!("ModelPixelScale: {:?}", tag_lookup.get_mut(&33550u16).expect("No ModelPixelScale!").resolve(&byte_order, &mut reader));
            println!("GEOTAGS\n---------------");
            println!("GeoKeyDirectory: {:?}", tag_lookup.get_mut(&34735u16).expect("No GeoKeyDirectory!").resolve(&byte_order, &mut reader));
            // println!("GeoDoubleParams: {:?}", tag_lookup.get_mut(&34736u16).expect("No GeoDoubleParams!").resolve(&byte_order, &mut reader));
            println!("GeoAsciiParams: {:?}", tag_lookup.get_mut(&34737u16).expect("No GeoAsciiParams!").resolve(&byte_order, &mut reader));

            let geo_ascii = match tag_lookup.get_mut(&34737u16) {
                Some(ascii) => match ascii.resolve(&byte_order, &mut reader) {
                    Ok(ascii) => match ascii {
                        EntryValue::ASCII(values) => match values.get(0) {
                            Some(v) => v.clone(),
                            None => {
                                return Err(InvalidOrUnhandledFormat(descriptor));
                            }
                        },
                        _ => {
                            return Err(InvalidOrUnhandledFormat(descriptor));
                        }
                    },
                    Err(_) => {
                        return Err(InvalidOrUnhandledFormat(descriptor));
                    }
                },
                None => {
                    println!("FOUND NO GEODATA");
                    return Err(NoGeoData(descriptor));
                }
            };
            let top_left = match tag_lookup.get_mut(&33922u16) {
                Some(t) => match t.resolve(&byte_order, &mut reader) {
                    Ok(v) => match v {
                        EntryValue::DOUBLE(values) => (match values.get(3) {
                            Some(v) => v.clone(),
                            None => {
                                return Err(InvalidOrUnhandledFormat(descriptor));
                            }
                        }, match values.get(4) {
                            Some(v) => v.clone(),
                            None => {
                                return Err(InvalidOrUnhandledFormat(descriptor));
                            }
                        }),
                        _ => {
                            return Err(InvalidOrUnhandledFormat(descriptor));
                        }
                    }
                    Err(_) => {
                        return Err(InvalidOrUnhandledFormat(descriptor));
                    }
                },
                None => {
                    return Err(NoGeoData(descriptor));
                }
            };
            let scale = match tag_lookup.get_mut(&33550u16) {
                Some(t) => match t.resolve(&byte_order, &mut reader) {
                    Ok(v) => match v {
                        EntryValue::DOUBLE(values) => (match values.get(0) {
                            Some(v) => v.clone(),
                            None => {
                                return Err(InvalidOrUnhandledFormat(descriptor));
                            }
                        }, match values.get(1) {
                            Some(v) => v.clone(),
                            None => {
                                return Err(InvalidOrUnhandledFormat(descriptor));
                            }
                        }),
                        _ => {
                            return Err(InvalidOrUnhandledFormat(descriptor));
                        }
                    }
                    Err(_) => {
                        return Err(InvalidOrUnhandledFormat(descriptor));
                    }
                },
                None => {
                    return Err(NoGeoData(descriptor));
                }
            };
            let image_dimensions = (match tag_lookup.get_mut(&256u16) {
                Some(t) => match t.resolve(&byte_order, &mut reader) {
                    Ok(v) => match v {
                        EntryValue::SHORT(values) => match values.get(0) {
                            Some(v) => v.clone(),
                            None => {
                                return Err(InvalidOrUnhandledFormat(descriptor));
                            }
                        },
                        _ => {
                            return Err(InvalidOrUnhandledFormat(descriptor));
                        }
                    }
                    Err(_) => {
                        return Err(InvalidOrUnhandledFormat(descriptor));
                    }
                },
                None => {
                    return Err(NoGeoData(descriptor));
                }
            }, match tag_lookup.get_mut(&257u16) {
                Some(t) => match t.resolve(&byte_order, &mut reader) {
                    Ok(v) => match v {
                        EntryValue::SHORT(values) => match values.get(0) {
                            Some(v) => v.clone(),
                            None => {
                                return Err(InvalidOrUnhandledFormat(descriptor));
                            }
                        },
                        _ => {
                            return Err(InvalidOrUnhandledFormat(descriptor));
                        }
                    }
                    Err(_) => {
                        return Err(InvalidOrUnhandledFormat(descriptor));
                    }
                },
                None => {
                    return Err(NoGeoData(descriptor));
                }
            });
            let proj_build = match Proj::new_known_crs(&geo_ascii, "EPSG:4326", None) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("FOO: {:?}", e);
                    return Err(NoGeoData(descriptor));
                }
            };

            eprintln!("FOO: {:?}", proj_build);

            exit(0);




            println!("{}", geo_ascii);
            if geo_ascii == "British National Grid (ORD SURV GB)|OSGB 1936|British National Grid (ORD SURV GB)|\0" {
                println!("FOUND BRITISH NATIONAL GRID FILE!");
            } else if (geo_ascii == "WGS 84|\0") {
                let bottom_right = (
                    (top_left.0 + (image_dimensions.0.clone() as f64) * scale.0),
                    (top_left.1 - (image_dimensions.1.clone() as f64) * scale.1)
                );

                let top_right = (
                    (top_left.0 + (bottom_right.0 - top_left.0)),
                    top_left.1
                );

                let bottom_left = (
                    (bottom_right.0 - (bottom_right.0 - top_left.0)),
                    bottom_right.1
                );
            } else if (geo_ascii == "OSGB 1936 / British National Grid|OSGB 1936|\0") {
                println!("FOUND OSGB 1936");
            } else {
                eprintln!("NO COORD SYSTEM GIVEN");
                panic!();
            }








            let bottom_right = (
                (top_left.0 + (image_dimensions.0.clone() as f64) * scale.0),
                (top_left.1 - (image_dimensions.1.clone() as f64) * scale.1)
            );

            let top_right = (
                (top_left.0 + (bottom_right.0 - top_left.0)),
                top_left.1
            );

            let bottom_left = (
                (bottom_right.0 - (bottom_right.0 - top_left.0)),
                bottom_right.1
            );

            println!("Got bounding box: {:?} :: {:?}", bottom_left, top_right);


            println!("Next IFD: {:?}", next_ifd);

            return Ok(Region{
                bottom_left: (0.0,0.0),
                top_right: (0.0,0.0)
            })
        },
        Err(e) => Err(ParsingErrorState::FileError(descriptor, e.kind()))
    }
}