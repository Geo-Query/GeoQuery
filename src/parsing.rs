use std::path::PathBuf;
use geotiff::{parse_tiff, FileDescriptor};
use crate::spatial::Region;
use crate::kml::parse_kml;

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

impl FileDescriptor for Descriptor {
    fn get_path(&self) -> &PathBuf {
        &self.path
    }
}

pub enum ParsingErrorState {
    UnknownExtension(Descriptor),
    NoExtension(Descriptor),
    FileError(Descriptor, std::io::ErrorKind),
    InvalidOrUnhandledFormat(Descriptor),
    NoGeoData(Descriptor)
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


pub fn parse_from_descriptor(descriptor: Descriptor) -> () {
    match &descriptor.data_type {
        Some(t) => match t {
            DataType::Kml => {
                parse_kml(descriptor);
            },
            DataType::TIFF => {
                parse_tiff(Box::new(descriptor));
            },
            DataType::Unknown(_) => {
                eprintln!("Unimplemented data type! {:?}", &descriptor.data_type)
            }
        },
        None => {
            eprintln!("No data type! {:?}", &descriptor.data_type)
        }
    }
}