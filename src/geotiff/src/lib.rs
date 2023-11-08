use std::collections::HashMap;
use std::fmt::format;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;
use proj::Proj;
use crate::entry::{EntryValue, IFDEntry};
use crate::geokeydirectory::GeoKeyDirectory;
pub use crate::header::HeaderErrorState;
pub use crate::entry::IFDEntryErrorState;
use crate::util::FromBytes;

mod util;
mod entry;
mod header;
mod geokeydirectory;
mod projections;

pub enum TIFFErrorState {
    HeaderError(HeaderErrorState),
    IFDEntryError(IFDEntryErrorState),
    NotEnoughGeoData,
    UnexpectedFormat(String)
}

pub trait FileDescriptor {
    fn get_path(&self) -> &PathBuf;
}


#[derive(Debug)]
pub struct GeoTiffRegion {
    pub bottom_left: (f64, f64),
    pub top_right: (f64, f64)
}

pub fn parse_tiff(reader: &mut BufReader<File>) -> Result<Box<GeoTiffRegion>, TIFFErrorState> {
    // Parse the file header.
    // First, seek to the start of the file, and validate.
    // Then read into an 8 byte buffer, and validate.
    // Then pass this buffer into parse_header. (unwrap as is using TIFFErrorState <3)
    let (byte_order, initial_ifd_offset) = match reader.seek(SeekFrom::Start(0)) {
        Ok(_) => {
            let mut header_buf = [0u8; 8];
            match reader.read_exact(&mut header_buf) {
                Ok(_) => {
                    header::parse_header(&header_buf)?
                },
                Err(e) => return Err(TIFFErrorState::UnexpectedFormat(String::from(format!("Read of header failed: {:?}", e))))
            }

        }
        Err(e) => return Err(TIFFErrorState::UnexpectedFormat(String::from(format!("Seek to header failed: {:?}", e))))
    };

    // Parse the IFD header; (Get entry count)
    // First, seek to start of header.
    // Then read into 2 byte buffer, and validate.
    // Then parse this buf as a u16, and and return.
    let entry_count = match reader.seek(initial_ifd_offset) {
        Ok(_) => {
            let mut entry_count_buf = [0u8; 2];
            match reader.read_exact(&mut entry_count_buf) {
                Ok(_) => {
                    u16::from_bytes(&entry_count_buf, &byte_order)
                },
                Err(e) => return Err(TIFFErrorState::UnexpectedFormat(String::from(format!("Read of IFD entry count failed: {:?}", e))))
            }
        },
        Err(e) => return Err(TIFFErrorState::UnexpectedFormat(String::from(format!("Seek to IFD failed: {:?}", e))))
    };

    // Init hashmap for entries.
    let mut entries: HashMap<u16, IFDEntry> = HashMap::with_capacity(entry_count as usize);

    for entry_number in 0..entry_count {
        let mut entry_buf = [0u8; 12];
        match reader.read_exact(&mut entry_buf) {
            Ok(_) => {
                let entry = IFDEntry::new(&entry_buf, &byte_order)?;
                entries.insert(entry.tag, entry);
            },
            Err(e) => return Err(TIFFErrorState::UnexpectedFormat(String::from(format!("Expected IFD Entry #{}, could not read, due to {:?}", entry_number, e))))
        }
    }

    // Read next 4 bytes for next ifd if you care.
    println!("Entries: {:?}", entries);

    let geo_key_directory = match entries.get_mut(&34735) {
        Some(v) => v.resolve(&byte_order, reader)?,
        None => return Err(TIFFErrorState::NotEnoughGeoData)
    };

    println!("GeoKeyDirectory: {:?}", geo_key_directory);

    return Ok(Box::new(GeoTiffRegion {
        bottom_left: (0.0, 0.0),
        top_right: (0.0, 0.0)
    }));
}

// pub fn parse_tiff_old(descriptor: Box<dyn FileDescriptor>) -> Result<Region, TIFFErrorState> {
//     match File::open(&descriptor.get_path()) {
//         Ok(file_handle) => {
//             let mut reader = BufReader::new(file_handle);
//             let (byte_order, offset) = match header::parse_header(&mut reader) {
//                 Ok((byte_order, offset)) => (byte_order, offset),
//                 Err(e) => {
//                     eprintln!("Encountered Error! {:?} Invalid Header!", e);
//                     panic!();
//                 }
//             };
//             println!("Byte Order: {:?}, Offset: {}", byte_order, offset);
//             let (mut entries, next_ifd) = match ifd::parse_ifd(&mut reader, SeekFrom::Start(offset as u64), &byte_order) {
//                 Ok(x) => x,
//                 Err(e) => {
//                     eprintln!("Encountered Error! {:?} Invalid IFD!", e);
//                     panic!();
//                 }
//             };
//             println!("Entries: {:?}", entries);
//             let mut tag_lookup: HashMap<u16, IFDEntry> = entries.into_iter().map(|entry| (entry.tag, entry)).collect();
//             let geo_key_directory = match tag_lookup.get_mut(&34735u16) {
//                 Some(e) => match e.resolve(&byte_order, &mut reader) {
//                     Ok(v) => match v {
//                         EntryValue::SHORT(v) => match GeoKeyDirectory::from_shorts(v.clone()) {
//                             Ok(d) => d,
//                             Err(e) => {
//                                 eprintln!("Error Parsing GeoKeyDirectory: {:?}", e);
//                                 return Err(UnexpectedFormat(String::from("Failed to parse GeoKeyDirectory")));
//                             }
//                         },
//                         _ => {
//                             return Err(UnexpectedFormat(String::from("Invalid value for GeoKeyDirectory (not shorts!)")));
//                         }
//                     },
//                     Err(e) => {
//                         eprintln!("Error Resolving GeoKeyDirectory: {:?}", e);
//                         return Err(UnexpectedFormat(String::from("Failed to resolve GeoKeyDirectory")));
//                     }
//                 },
//                 _ => {
//                     return Err(UnexpectedFormat(String::from("No GeoKeyDirectory")));
//                 }
//             };
//
//             println!("GeoKeyDirectory: {:?}", geo_key_directory);
//
//             let proj = match projections::get_proj_from_key_directory(geo_key_directory, &mut tag_lookup, &byte_order, &mut reader, "EPSG:4326") {
//                 Ok(proj) => proj,
//                 Err(e) => {
//                     eprintln!("Got error: {:?}", e);
//                     panic!();
//                 }
//             };
//
//             let top_left = match tag_lookup.get_mut(&33922u16) {
//                 Some(t) => match t.resolve(&byte_order, &mut reader) {
//                     Ok(v) => match v {
//                         EntryValue::DOUBLE(values) => (match values.get(3) {
//                             Some(v) => v.clone(),
//                             None => {
//                                 return Err(UnexpectedFormat(String::from("No ModelTiePoint")));
//                             }
//                         }, match values.get(4) {
//                             Some(v) => v.clone(),
//                             None => {
//                                 return Err(UnexpectedFormat(String::from("No ModelTiePoint")));
//                             }
//                         }),
//                         _ => {
//                             return Err(UnexpectedFormat(String::from("No ModelTiePoint")));
//                         }
//                     }
//                     Err(_) => {
//                         return Err(UnexpectedFormat(String::from("No ModelTiePoint")));
//                     }
//                 },
//                 None => {
//                     return Err(UnexpectedFormat(String::from("No ModelTiePoint")));
//                 }
//             };
//
//             let scale = match tag_lookup.get_mut(&33550u16) {
//                 Some(t) => match t.resolve(&byte_order, &mut reader) {
//                     Ok(v) => match v {
//                         EntryValue::DOUBLE(values) => (match values.get(0) {
//                             Some(v) => v.clone(),
//                             None => {
//                                 return Err(UnexpectedFormat(String::from("No Scale")));
//                             }
//                         }, match values.get(1) {
//                             Some(v) => v.clone(),
//                             None => {
//                                 return Err(UnexpectedFormat(String::from("No Scale")));
//                             }
//                         }),
//                         _ => {
//                             return Err(UnexpectedFormat(String::from("No Scale")));
//                         }
//                     }
//                     Err(_) => {
//                         return Err(UnexpectedFormat(String::from("No Scale")));
//                     }
//                 },
//                 None => {
//                     return Err(UnexpectedFormat(String::from("No Scale")));
//                 }
//             };
//
//             let image_dimensions = (match tag_lookup.get_mut(&256u16) {
//                     Some(t) => match t.resolve(&byte_order, &mut reader) {
//                         Ok(v) => match v {
//                             EntryValue::SHORT(values) => match values.get(0) {
//                                 Some(v) => v.clone(),
//                                 None => {
//                                     return Err(UnexpectedFormat(String::from("No Dim X")));
//                                 }
//                             },
//                             _ => {
//                                 return Err(UnexpectedFormat(String::from("No Dim X")));
//                             }
//                         }
//                         Err(_) => {
//                             return Err(UnexpectedFormat(String::from("No Dim X")));
//                         }
//                     },
//                     None => {
//                         return Err(UnexpectedFormat(String::from("No Dim X")));
//                     }
//                 }, match tag_lookup.get_mut(&257u16) {
//                     Some(t) => match t.resolve(&byte_order, &mut reader) {
//                         Ok(v) => match v {
//                             EntryValue::SHORT(values) => match values.get(0) {
//                                 Some(v) => v.clone(),
//                                 None => {
//                                     return Err(UnexpectedFormat(String::from("No Dim Y")));
//                                 }
//                             },
//                             _ => {
//                                 return Err(UnexpectedFormat(String::from("No Dim Y")));
//                             }
//                         }
//                         Err(_) => {
//                             return Err(UnexpectedFormat(String::from("No Dim Y")));
//                         }
//                     },
//                     None => {
//                         return Err(UnexpectedFormat(String::from("No Dim Y")));
//                     }
//                 });
//
//
//
//
//             calculate_extent(top_left, scale, image_dimensions, proj);
//
//
//
//
//             println!("NEXTIFD: {:?}", next_ifd);
//             println!("ImageWidth: {:?}", tag_lookup.get_mut(&256u16).expect("No ImageWidth Entry!").resolve(&byte_order, &mut reader));
//             println!("ImageLength: {:?}", tag_lookup.get_mut(&257u16).expect("No ImageLength Entry!").resolve(&byte_order, &mut reader));
//             println!("ModelTiePoint: {:?}", tag_lookup.get_mut(&33922u16).expect("No ModelTiePoint Entry!").resolve(&byte_order, &mut reader));
//             println!("ModelPixelScale: {:?}", tag_lookup.get_mut(&33550u16).expect("No ModelPixelScale!").resolve(&byte_order, &mut reader));
//             println!("GEOTAGS\n---------------");
//             println!("GeoKeyDirectory: {:?}", tag_lookup.get_mut(&34735u16).expect("No GeoKeyDirectory!").resolve(&byte_order, &mut reader));
//             // println!("GeoDoubleParams: {:?}", tag_lookup.get_mut(&34736u16).expect("No GeoDoubleParams!").resolve(&byte_order, &mut reader));
//             println!("GeoAsciiParams: {:?}", tag_lookup.get_mut(&34737u16).expect("No GeoAsciiParams!").resolve(&byte_order, &mut reader));
//
//
//             let geo_ascii = match tag_lookup.get_mut(&34737u16) {
//                 Some(ascii) => match ascii.resolve(&byte_order, &mut reader) {
//                     Ok(ascii) => match ascii {
//                         EntryValue::ASCII(values) => match values.get(0) {
//                             Some(v) => v.clone(),
//                             None => {
//                                 return Err(UnexpectedFormat(String::from("No GeoAscii Params")));
//                             }
//                         },
//                         _ => {
//                             return Err(UnexpectedFormat(String::from("No GeoAscii Params")));
//                         }
//                     },
//                     Err(_) => {
//                         return Err(UnexpectedFormat(String::from("No GeoAscii Params")));
//                     }
//                 },
//                 None => {
//                     println!("FOUND NO GEODATA");
//                     return Err(UnexpectedFormat(String::from("No GeoAscii Params")));
//                 }
//             };
//
//             let scale = match tag_lookup.get_mut(&33550u16) {
//                 Some(t) => match t.resolve(&byte_order, &mut reader) {
//                     Ok(v) => match v {
//                         EntryValue::DOUBLE(values) => (match values.get(0) {
//                             Some(v) => v.clone(),
//                             None => {
//                                 return Err(UnexpectedFormat(String::from("No Scale")));
//                             }
//                         }, match values.get(1) {
//                             Some(v) => v.clone(),
//                             None => {
//                                 return Err(UnexpectedFormat(String::from("No Scale")));
//                             }
//                         }),
//                         _ => {
//                             return Err(UnexpectedFormat(String::from("No Scale")));
//                         }
//                     }
//                     Err(_) => {
//                         return Err(UnexpectedFormat(String::from("No Scale")));
//                     }
//                 },
//                 None => {
//                     return Err(UnexpectedFormat(String::from("No Scale")));
//                 }
//             };
//             //
//             // let proj_build = match Proj::new_known_crs(&geo_ascii, "EPSG:4326", None) {
//             //     Ok(v) => v,
//             //     Err(e) => {
//             //         eprintln!("FOO: {:?}", e);
//             //         return Err(NoGeoData(descriptor));
//             //     }
//             // };
//             //
//             // eprintln!("FOO: {:?}", proj_build);
//             //
//             //
//             //
//             //
//             //
//             // println!("{}", geo_ascii);
//             // if geo_ascii == "British National Grid (ORD SURV GB)|OSGB 1936|British National Grid (ORD SURV GB)|\0" {
//             //     println!("FOUND BRITISH NATIONAL GRID FILE!");
//             // } else if (geo_ascii == "WGS 84|\0") {
//             //     let bottom_right = (
//             //         (top_left.0 + (image_dimensions.0.clone() as f64) * scale.0),
//             //         (top_left.1 - (image_dimensions.1.clone() as f64) * scale.1)
//             //     );
//             //
//             //     let top_right = (
//             //         (top_left.0 + (bottom_right.0 - top_left.0)),
//             //         top_left.1
//             //     );
//             //
//             //     let bottom_left = (
//             //         (bottom_right.0 - (bottom_right.0 - top_left.0)),
//             //         bottom_right.1
//             //     );
//             // } else if (geo_ascii == "OSGB 1936 / British National Grid|OSGB 1936|\0") {
//             //     println!("FOUND OSGB 1936");
//             // } else {
//             //     eprintln!("NO COORD SYSTEM GIVEN");
//             //     panic!();
//             // }
//             //
//             //
//             //
//             //
//             //
//             //
//             //
//             //
//             // let bottom_right = (
//             //     (top_left.0 + (image_dimensions.0.clone() as f64) * scale.0),
//             //     (top_left.1 - (image_dimensions.1.clone() as f64) * scale.1)
//             // );
//             //
//             // let top_right = (
//             //     (top_left.0 + (bottom_right.0 - top_left.0)),
//             //     top_left.1
//             // );
//             //
//             // let bottom_left = (
//             //     (bottom_right.0 - (bottom_right.0 - top_left.0)),
//             //     bottom_right.1
//             // );
//             //
//             // println!("Got bounding box: {:?} :: {:?}", bottom_left, top_right);
//             //
//             //
//             // println!("Next IFD: {:?}", next_ifd);
//
//             return Ok(Region{
//                 bottom_left: (0.0,0.0),
//                 top_right: (0.0,0.0)
//             })
//         },
//         Err(e) => {
//             eprintln!("{:?}", e);
//             return Err(TIFFErrorState::FailedToParseTag)
//         }
//     }
// }
fn calculate_extent(
    top_left: (f64, f64),
    scale: (f64, f64),
    image_dimensions: (u16, u16),
    proj: Proj
) -> () {
    // Initialize the Proj struct with the known CRS (Coordinate Reference System)

    // Calculate the bottom-right coordinates in the image's CRS
    let bottom_right = (
        top_left.0 + (scale.0 * image_dimensions.0 as f64),
        top_left.1 - (scale.1 * image_dimensions.1 as f64), // subtract because pixel scale is usually positive as you go down
    );

    // Convert the top-left and bottom-right coordinates to latitude and longitude
    let top_left_lat_long = proj.convert((top_left.0, top_left.1));
    let bottom_right_lat_long = proj.convert((bottom_right.0, bottom_right.1));

    // Print out the extents in latitude and longitude
    println!("Top Left Latitude/Longitude: {:?}", top_left_lat_long);
    println!("Bottom Right Latitude/Longitude: {:?}", bottom_right_lat_long);
}
