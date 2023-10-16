use std::path::PathBuf;
use crate::parsing::{ParsingErrorState};
mod kml;
mod util;
mod spatial;
mod index;
mod parsing;

fn main() {
    let kml_file = parsing::Descriptor::new(PathBuf::from("/home/ben/uni/psd/teamproj/sh35-data-parsing/luciad_and_leuven.kml"));
    match parsing::parse_from_descriptor(kml_file) {
        Ok(region) => {
            println!("Result: {:?}", region);
        },
        Err(error_kind) => match error_kind {
            ParsingErrorState::UnknownExtension(descriptor) => {
                eprintln!("Encountered unknown extension {:?}", descriptor.data_type);
                panic!();
            },
            ParsingErrorState::NoExtension(descriptor) => {
                eprintln!("No file extension for file: {:?}", descriptor);
                panic!();
            },
            ParsingErrorState::FileError(descriptor, error_kind) => {
                eprintln!("Failed to open: {:?}", descriptor);
                eprintln!("ErrorKind: {:?}", error_kind);
                panic!();
            }
        }
    };
}
