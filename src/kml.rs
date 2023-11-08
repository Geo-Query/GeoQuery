use crate::spatial::{Coordinate, Region};
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use xml::reader::{EventReader, XmlEvent};
use crate::kml::KMLErrorState::{NotEnoughGeoData, UnexpectedFormat};

pub fn get_boundaries(coordinates: Vec<Coordinate>) -> (Coordinate, Coordinate) {
    let mut min_x: f64 = coordinates[0].0;
    let mut min_y: f64 = coordinates[0].1;
    let mut max_x: f64 = coordinates[0].0;
    let mut max_y: f64 = coordinates[0].1;
    for coordinate in coordinates {
        if coordinate.0 > max_x {
            max_x = coordinate.0;
        }
        if coordinate.0 < min_x {
            min_x = coordinate.0;
        }
        if coordinate.1 > max_y {
            max_y = coordinate.1;
        }
        if coordinate.1 < min_y {
            min_y = coordinate.1;
        }
    }
    return ((min_x, min_y), (max_x, max_y));
}

#[derive(Debug)]
pub struct KMLRegion {
    pub bottom_left: Coordinate,
    pub top_right: Coordinate
}

impl Region for KMLRegion {
    fn bottom_left(&self) -> Coordinate {
        self.bottom_left
    }

    fn bottom_right(&self) -> Coordinate {
        (self.top_right.0, self.bottom_left.1)
    }

    fn top_left(&self) -> Coordinate {
        (self.bottom_left.0, self.top_right.1)
    }

    fn top_right(&self) -> Coordinate {
        self.top_right
    }
}

pub enum KMLErrorState {
    UnexpectedFormat(String),
    NotEnoughGeoData
}

pub fn parse_kml(reader: &mut BufReader<File>) -> Result<Box<dyn Region>, KMLErrorState> {
    // Initialise Event iterator, as well as coordinate buffer.
    let mut reader = EventReader::new(reader).into_iter();
    let mut coordinates: Vec<(f64, f64)> = vec![];

    while let Some(Ok(event)) = reader.next() { // Capture events until file over.
        match event {
            XmlEvent::StartElement {name, ..} if name.local_name == "coordinates" => { // When Coordinate element starts...
                while let Some(Ok(event)) = reader.next() { // Start capturing all events until Coordinate element ends.
                    match event {
                        XmlEvent::Characters(_0) => { // While capturing, get all raw chars.
                            // Conform data into coordinate pairs...
                            let _0 = _0.replace("\n", "");
                            let coordinate_pairs = _0.split_whitespace(); // Split by whitespace, each coord set is space seperated.
                            for coordinate_pair in coordinate_pairs {
                                let coordinate_strs: Vec<&str> = coordinate_pair.split(",").collect();
                                if coordinate_strs.len() < 2 {
                                    return Err(UnexpectedFormat(String::from(format!("Expected coordinate pair of len 2, got: {:?}", coordinate_strs))));
                                }
                                coordinates.push((
                                    match f64::from_str(coordinate_strs[0]) {
                                        Ok(v) => v,
                                        Err(e) => {
                                            return Err(UnexpectedFormat(String::from(format!("Failed to parse floating point coord: {} with err: {:?}", coordinate_strs[0], e))));
                                        }
                                    }, match f64::from_str(coordinate_strs[1]) {
                                        Ok(v) => v,
                                        Err(e) => {
                                            return Err(UnexpectedFormat(String::from(format!("Failed to parse floating point coord: {} with err: {:?}", coordinate_strs[1], e))));
                                        }
                                    }
                                ));
                            }
                        },
                        XmlEvent::EndElement {name} if name.local_name == "coordinates" => break, // Handle end of coordinate element
                        _ => {} // Ignore contained elems
                    }
                }
            }
            _ => {} // Ignore all but start elem.
        }
    }

    if coordinates.len() == 0 {
        return Err(NotEnoughGeoData);
    }

    let (bottom_left, top_right) = get_boundaries(coordinates); // Draw a bounding box around given coords
    return Ok(Box::new(KMLRegion {
        bottom_left,
        top_right
    })); // Return region defined by file.
}