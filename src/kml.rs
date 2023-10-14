use crate::data::MapFileDescriptor;
use std::fs::{File, read};
use xml::EventReader;
use xml::reader::XmlEvent;

pub fn parse_kml_file(file_descriptor: MapFileDescriptor) {
    println!("Parsing!");
    let file_handle = match File::open(file_descriptor.file_path) {
        Ok(file_handle) => file_handle,
        Err(_) => {
            eprintln!("Failed to open file handle! Exiting!");
            panic!();
        }
    };

    let file_reader = EventReader::new(file_handle);
    let mut file_reader_iter = file_reader.into_iter();

    while let Some(Ok(event)) = file_reader_iter.next() {
        match event {
            XmlEvent::StartElement {name, ..} if name.local_name == "coordinates" => {
                println!("Started Coordinate Capture");
                while let Some(Ok(event)) = file_reader_iter.next() {
                    match event {
                        XmlEvent::Characters(_0) => {
                            println!("Coordinates: {}", _0);
                        },
                        XmlEvent::EndElement {name} if name.local_name == "coordinates" => break,
                        _ => {
                            println!("Unexpected event within coord block: {:?}", event);
                        }
                    }
                }
                println!("Coordinate capture ended!")
            }
            _ => {}
        }
    }
}