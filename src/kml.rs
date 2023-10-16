use crate::parsing::{Descriptor, ParsingErrorState};
use crate::spatial::{Coordinate, Region};
use crate::util::draw_boundaries;
use std::fs::File;
use std::str::FromStr;
use xml::reader::{EventReader, XmlEvent};

pub fn parse_kml(descriptor: Descriptor) -> Result<Region, ParsingErrorState> {
    println!("Entered!");
    match File::open(&descriptor.path) { // Open file handle, throw error if failure.
        Ok(file) => {
            let mut file_reader = EventReader::new(file).into_iter(); // Instantiate file reader.
            let mut coordinates: Vec<Coordinate> = vec![]; // Create collection of coords.
            while let Some(Ok(event)) = file_reader.next() { // Capture events until file over.
                match event { // Wait for StartElement (coordinates)
                    XmlEvent::StartElement {name, ..} if name.local_name == "coordinates" => { // If elem is coords,
                        while let Some(Ok(event)) = file_reader.next() { // Capture all events, (Until ends, see EndElement handler)
                            match event { // Match event type (Characters or EndElement (coordinates))
                                XmlEvent::Characters(_0) => { // If chars, parse.
                                    let _0 = _0.replace("\n", ""); // Strip newlines
                                    let coordinate_sets = _0.split_whitespace(); // Split by whitespace, dividing into coordinate_sets.
                                    for coordinate_set in coordinate_sets {
                                        let coordinate_set = coordinate_set.split(",").take(2); // Split by commas, dividing into coordinate values.
                                        let mut coordinate_set = coordinate_set.map(|v| match f64::from_str(v) {
                                            Ok(v) => v, // If can be converted to float; just return.
                                            Err(_) => {
                                                // If can't, trigger a panic.
                                                // This is reasonable because any non-conformity of file-types
                                                // needs to be handled at this point.
                                                println!("Failed to parse coordinate in file: {:?}", descriptor);
                                                println!("Value attempted to parse: {}", v);
                                                panic!();
                                            }
                                        });
                                        coordinates.push( // Push coords to collected vector; or throw error and panic if one missing.
                                            if let (Some(x), Some(y)) = (coordinate_set.next(), coordinate_set.next()) {
                                                (x, y)
                                            } else {
                                                println!("Failed to parse coordinate set in file: {:?}", descriptor);
                                                println!("Missing X, or Y, coordinate");
                                                panic!();
                                            }
                                        )
                                    }
                                },
                                XmlEvent::EndElement {name} if name.local_name == "coordinates" => break,
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            };
            let (bottom_left, top_right) = draw_boundaries(coordinates); // Draw a bounding box around given coords
            return Ok(Region {
                bottom_left,
                top_right
            }); // Return region defined by file.
        },
        Err(e) => Err(ParsingErrorState::FileError(descriptor, e.kind()))
    }
}