use std::path::PathBuf;
use crate::spatial::Region;
use crate::kml::parse_kml;
use crate::tiff::parse_tiff;

#[derive(Debug)]
pub enum DataType {
    Kml,
    TIFF,
    Unknown(String)
}

#[derive(Debug)]
pub struct Descriptor {
    pub path: PathBuf,
    pub data_type: Option<DataType>
}

pub enum ParsingErrorState {
    UnknownExtension(Descriptor),
    NoExtension(Descriptor),
    FileError(Descriptor, std::io::ErrorKind)
}

impl Descriptor {
    pub fn new(path: PathBuf) -> Descriptor {
        let p = path.clone();
        return Descriptor {
            path,
            data_type: match p.extension() {
                Some(ext) => match ext.to_str().unwrap() {
                    "kml" => Some(DataType::Kml),
                    _ => Some(DataType::Unknown(String::from(ext.to_str().unwrap())))
                },
                None => None
            }
        }
    }
}


pub fn parse_from_descriptor(descriptor: Descriptor) -> Result<Region, ParsingErrorState> {
    return match &descriptor.data_type {
        Some(t) => match t {
            DataType::Kml => parse_kml(descriptor),
            DataType::TIFF => parse_tiff(descriptor),
            DataType::Unknown(_) => Err(ParsingErrorState::UnknownExtension(descriptor))
        },
        None => Err(ParsingErrorState::NoExtension(descriptor))
    }
}