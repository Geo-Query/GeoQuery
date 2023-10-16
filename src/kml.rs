use crate::data::{draw_boundaries, MapData, MapFileDescriptor};
use crate::region::{Region};
use std::fs::File;
use std::iter::Map;
use std::str::FromStr;
use rstar::RTreeNode::Parent;
use xml::EventReader;
use xml::reader::XmlEvent;

pub fn parse_kml_file(file_descriptor: MapFileDescriptor) -> MapData {
    let mut coordinates: Vec<[f64; 2]> = vec![];
    match File::open(&file_descriptor.file_path) {
        Ok(file_handle) => {
            let mut file_reader = EventReader::new(file_handle).into_iter();
            while let Some(Ok(event)) = file_reader.next() {
                match event {
                    XmlEvent::StartElement {name, ..} if name.local_name == "coordinates" => {
                        while let Some(Ok(event)) = file_reader.next() {
                            match event {
                                XmlEvent::Characters(_0) => {
                                    let _0 = _0.replace("\n", "");
                                    let _0 = _0.split_whitespace();
                                    for coordinate_set in _0 {
                                        let set = coordinate_set.split(",");
                                        let mut set = set.take(2).map(|v| match f64::from_str(v) {
                                            Ok(v) => v,
                                            Err(_) => {
                                                println!("Can't parse into f64! {}", coordinate_set);
                                                panic!();
                                            }
                                        });
                                        coordinates.push([match set.next() {
                                            Some(x) => x,
                                            None => {
                                                println!("Not enough values, coordinate pair has no X value!");
                                                panic!();
                                            }
                                        }, match set.next() {
                                            Some(y) => y,
                                            None => {
                                                println!("Not enough values, coordinate pair has no Y value!");
                                                panic!();
                                            }
                                        }]);
                                    };

                                },
                                XmlEvent::EndElement {name} if name.local_name == "coordinates" => break,
                                _ => {}
                            }
                        }
                    },
                    _ => {}
                }
            }
        },
        Err(_) => {
            eprintln!("Failed to open file handle {:?}", file_descriptor.file_path);
            panic!();
        }
    };
    let (bottom_left, top_right) = draw_boundaries(coordinates);
    return MapData {
        handle:file_descriptor,
        region: Region {
            bottom_left,
            top_right
        },
    };
}