use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;
use proj4rs::Proj;
use error::TIFFErrorState;
use crate::entry::{EntryValue, IFDEntry};
pub use error::GeoKeyDirectoryErrorState;
pub use error::HeaderErrorState;
pub use error::IFDEntryErrorState;
use crate::geokeydirectory::GeoKeyDirectory;
use error::TIFFErrorState::ProjectionError;
use serde::{Deserialize, Serialize};
use crate::tfw::parse_tfw;
use crate::util::FromBytes;


mod util;
mod entry;
mod header;
mod geokeydirectory;
mod error;
mod tfw;

pub trait FileDescriptor {
    fn get_path(&self) -> &PathBuf;
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoTiffMap {
    pub tiff: PathBuf,
    pub tfw: Option<PathBuf>,
    pub prj: Option<PathBuf>
}

#[derive(Debug, Clone)]
pub struct GeoTiffRegion {
    pub top_left: (f64, f64),
    pub bottom_right: (f64, f64),
}

#[derive(Debug, Clone)]
pub struct GeoTiffMetaData {
    pub region: GeoTiffRegion,
    pub tags: Vec<(String, String)>
}

pub fn parse_tiff(reader: &mut BufReader<File>, tfw_reader: Option<&mut BufReader<File>>) -> Result<GeoTiffMetaData, TIFFErrorState> {
    if tfw_reader.is_some() {
        parse_tfw(&mut tfw_reader.unwrap());
    }




    let tags = vec![("Filetype".to_string(), "TIFF".to_string())];
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

    // let projection = geo_key_directory.get_projection("EPSG:4326")?;
    let projection = geo_key_directory.get_projection()?;

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

    return Ok(GeoTiffMetaData {
        region,
        tags
    });
}

fn calculate_extent(
    top_left: (f64, f64),
    scale: (f64, f64),
    image_dimensions: (u16, u16),
    from_proj: Proj
) -> Result<GeoTiffRegion, TIFFErrorState> {
    // Initialize the Proj struct with the known CRS (Coordinate Reference System)
    let to_proj = Proj::from_proj_string(crs_definitions::EPSG_4326.proj4).expect("FAILED TO BUILD DEFAULT PROJ!");

    let mut top_left = top_left.clone();
    // Calculate the bottom-right coordinates in the image's CRS
    let mut bottom_right = (
        top_left.0 + (scale.0 * image_dimensions.0 as f64),
        top_left.1 - (scale.1 * image_dimensions.1 as f64), // subtract because pixel scale is usually positive as you go down
    );

    if let Err(e) = proj4rs::transform::transform(&from_proj, &to_proj, &mut top_left) {
        return Err(ProjectionError(format!("Failed to apply tranformation for {from_proj:?} to {to_proj:?}, for points: {top_left:?}, with reason {e:?}")))
    } else {};
    if let Err(e) = proj4rs::transform::transform(&from_proj, &to_proj, &mut bottom_right) {
        return Err(ProjectionError(format!("Failed to apply tranformation for {from_proj:?} to {to_proj:?}, for points: {bottom_right:?}, with reason {e:?}")))
    } else {};


    return Ok(GeoTiffRegion {
        top_left,
        bottom_right
    });
}


 #[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Write, Seek, SeekFrom};
    use tempfile::tempfile;
    use std::fs::File;
    use std::io::BufReader;
    use byteorder::{LittleEndian, WriteBytesExt,ByteOrder};
    fn mock_geo_tiff_data() -> Vec<u8> {
        let mut data = Vec::new();
        // Mock GeoKeyDirectory header data
        let geo_key_directory_header = vec![1u16, 1, 0, 1]; // Key Directory Version, Key Revision, Minor Revision, Number of Keys
        // Mock GeoKey: GeographicTypeGeoKey (ID 2048) set to EPSG:4326 (WGS 84)
        let geographic_type_geo_key = vec![2048u16, 0, 1, 4326]; // ID, Location, Count, Value

        // Write the header and key into data
        for value in geo_key_directory_header.iter().chain(geographic_type_geo_key.iter()) {
            data.write_u16::<LittleEndian>(*value).unwrap(); // Note: using unwrap() here; handle errors more gracefully in production code
        }

        data
    }

    #[test]
    fn test_geo_key_directory_parsing() {
        // Create mock data
        let mock_data = mock_geo_tiff_data();
        // Create a temporary file and write mock data into it
        let mut file = tempfile().expect("Failed to create temporary file");
        file.write_all(&mock_data).expect("Failed to write mock data to temporary file");
        file.seek(SeekFrom::Start(0)).expect("Failed to rewind temporary file");

        // Read the temporary file into a BufReader
        let reader = BufReader::new(file);
        // Read data from BufReader into a Vec<u16>
        let mut shorts = Vec::new();
        for chunk in mock_data.chunks(2) {
            shorts.push(LittleEndian::read_u16(&chunk));
        }

        // Attempt to parse the GeoKeyDirectory
        let result = GeoKeyDirectory::from_shorts(&shorts);
        assert!(result.is_ok(), "Failed to parse GeoKeyDirectory from mock data");

        // Further validate the GeoKeyDirectory
        let geo_key_directory = result.unwrap();
        assert_eq!(geo_key_directory.header.key_revision, 1);
        assert_eq!(geo_key_directory.header.minor_revision, 0);
        assert_eq!(geo_key_directory.header.count, 1);
        assert!(geo_key_directory.keys.contains_key(&2048), "GeoKeyDirectory does not contain expected GeoKey");
        assert_eq!(geo_key_directory.keys.get(&2048).unwrap().value, Some(4326), "GeographicTypeGeoKey does not match expected value");
    }

    #[test]
    fn test_geo_key_directory_incorrect_header_length() {
        // Create mock data with incorrect header length to simulate an error
        let incorrect_header = vec![1, 1, 0]; // Missing one element
        let result = GeoKeyDirectory::from_shorts(&incorrect_header);
        assert!(result.is_err(), "Expected an error due to incorrect header length");
    }

    #[test]
    fn test_geo_key_directory_unsupported_version() {
        // Create mock data with an unsupported version to simulate an error
        let unsupported_version_header = vec![0, 1, 0, 1]; // Version set to 0
        let result = GeoKeyDirectory::from_shorts(&unsupported_version_header);
        assert!(result.is_err(), "Expected an error due to unsupported GeoKeyDirectory version");
    }

}
