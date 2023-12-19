use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;
use proj::{Proj, ProjError};
use crate::entry::{EntryValue, IFDEntry};
pub use crate::geokeydirectory::GeoKeyDirectoryErrorState;
pub use crate::header::HeaderErrorState;
pub use crate::entry::IFDEntryErrorState;
use crate::geokeydirectory::GeoKeyDirectory;
use crate::util::FromBytes;

mod util;
mod entry;
mod header;
mod geokeydirectory;

pub enum TIFFErrorState {
    HeaderError(HeaderErrorState),
    IFDEntryError(IFDEntryErrorState),
    GeoKeyDirectoryError(GeoKeyDirectoryErrorState),
    UnexpectedFormat(String),
    ProjectionError(ProjError),
    NotEnoughGeoData,
}

pub trait FileDescriptor {
    fn get_path(&self) -> &PathBuf;
}


#[derive(Debug)]
pub struct GeoTiffRegion {
    pub top_left: (f64, f64),
    pub bottom_right: (f64, f64),
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

    // Read next 4 bytes for next ifd if you care
    // TODO: Implement support for multiple IFDs.
    // println!("Entries: {:?}", entries);


    let geo_key_directory = match entries.get_mut(&34735) {
        Some(v) => if let EntryValue::SHORT(v) = v.resolve(&byte_order, reader)?  {
            v
        } else {
            return Err(TIFFErrorState::UnexpectedFormat(String::from("Expected GeoKeyDirectory to be of type SHORT!")));
        },
        None => return Err(TIFFErrorState::NotEnoughGeoData)
    };

    let geo_key_directory = GeoKeyDirectory::from_shorts(geo_key_directory)?;

    // println!("GeoKeyDirectory: {:?}", geo_key_directory);

    let projection = geo_key_directory.get_projection("EPSG:4326")?;

    let top_left = match entries.get_mut(&33922) {
        None => return Err(TIFFErrorState::NotEnoughGeoData),
        Some(v) => if let EntryValue::DOUBLE(v) = v.resolve(&byte_order, reader)? {
            if let (Some(x), Some(y)) = (v.get(3), v.get(4)) {
                (x.clone(), y.clone())
            } else {
                return Err(TIFFErrorState::NotEnoughGeoData)
            }
        } else {
            return Err(TIFFErrorState::UnexpectedFormat(String::from("Expected ModelTiePoint to be of type DOUBLE!")));
        }
    };
    let scale =  match entries.get_mut(&33550) {
        None => return Err(TIFFErrorState::NotEnoughGeoData),
        Some(v) => if let EntryValue::DOUBLE(v) = v.resolve(&byte_order, reader)? {
            if let (Some(x), Some(y)) = (v.get(0), v.get(1)) {
                (x.clone(), y.clone())
            } else {
                return Err(TIFFErrorState::NotEnoughGeoData)
            }
        } else {
            return Err(TIFFErrorState::UnexpectedFormat(String::from("Expected ModelTiePoint to be of type DOUBLE!")));
        }
    };
    let x = if let Some(entry) = entries.get_mut(&256) {
        // Resolve the entry to a specific value (like width).
        if let EntryValue::SHORT(v) = entry.resolve(&byte_order, reader)? {
            if let Some(x) = v.get(0) {
                x.clone()
            } else {
                return Err(TIFFErrorState::UnexpectedFormat(String::from("Expected ImageWidth!")));
            }
        } else {
            return Err(TIFFErrorState::UnexpectedFormat(String::from("Expected ImageWidth to be of type SHORT!")));

        }
    } else {
        // If the entry for ImageWidth does not exist, return an error.
        return Err(TIFFErrorState::UnexpectedFormat(String::from("Expected ImageWidth!")));
    };

    let y = if let Some(entry) = entries.get_mut(&257) {
        // Resolve the entry to a specific value (like width).
        if let EntryValue::SHORT(v) = entry.resolve(&byte_order, reader)? {
            if let Some(y) = v.get(0) {
                y.clone()
            } else {
                return Err(TIFFErrorState::UnexpectedFormat(String::from("Expected ImageLength!")));
            }
        } else {
            return Err(TIFFErrorState::UnexpectedFormat(String::from("Expected ImageLength to be of type SHORT!")));

        }
    } else {
        // If the entry for ImageWidth does not exist, return an error.
        return Err(TIFFErrorState::UnexpectedFormat(String::from("Expected ImageLength!")));
    };

    let region = calculate_extent(top_left, scale, (x,y), projection)?;

    return Ok(Box::new(region));
}

fn calculate_extent(
    top_left: (f64, f64),
    scale: (f64, f64),
    image_dimensions: (u16, u16),
    proj: Proj
) -> Result<GeoTiffRegion, TIFFErrorState> {
    // Initialize the Proj struct with the known CRS (Coordinate Reference System)

    // Calculate the bottom-right coordinates in the image's CRS
    let bottom_right = (
        top_left.0 + (scale.0 * image_dimensions.0 as f64),
        top_left.1 - (scale.1 * image_dimensions.1 as f64), // subtract because pixel scale is usually positive as you go down
    );

    return Ok(GeoTiffRegion {
        top_left: match proj.convert(top_left) {
            Ok(v) => v,
            Err(e) => return Err(TIFFErrorState::ProjectionError(e))
        },
        bottom_right: match proj.convert(bottom_right) {
            Ok(v) => v,
            Err(e) => return Err(TIFFErrorState::ProjectionError(e))
        }
    });
}
