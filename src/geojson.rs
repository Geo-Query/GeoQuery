use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;
use crate::spatial::{Coordinate, Region};
use json_event_parser::{JsonReader, JsonEvent};
use proj::Coord;

pub fn get_boundaries(coordinates: Vec<[f64; 2]>) -> (Coordinate, Coordinate) {
    let mut min_x: f64 = coordinates[0][0];
    let mut min_y: f64 = coordinates[0][1];
    let mut max_x: f64 = coordinates[0][0];
    let mut max_y: f64 = coordinates[0][1];
    for coordinate in coordinates {
        if coordinate[0] > max_x {
            max_x = coordinate[0];
        }
        if coordinate[0] < min_x {
            min_x = coordinate[0];
        }
        if coordinate[1] > max_y {
            max_y = coordinate[1];
        }
        if coordinate[1] < min_y {
            min_y = coordinate[1];
        }
    }
    return ((min_x, min_y), (max_x, max_y));
}

#[derive(Debug)]
pub enum GeoJSONErrorState {
    InvalidJSON(Box<dyn Error>),
    UnparsableCoordinate(String)
}

#[derive(Debug)]
pub struct GeoJSONRegion {
    top_left: Coordinate,
    bottom_right: Coordinate
}

impl Region for GeoJSONRegion {
    fn bottom_left(&self) -> Coordinate {
        todo!()
    }

    fn bottom_right(&self) -> Coordinate {
        self.bottom_right
    }

    fn top_left(&self) -> Coordinate {
        self.top_left
    }

    fn top_right(&self) -> Coordinate {
        todo!()
    }
}

pub fn parse_geojson(reader: &mut BufReader<File>) -> Result<Box<GeoJSONRegion>, GeoJSONErrorState> {
    let mut json_reader = JsonReader::from_reader(reader);
    let mut buffer = Vec::new();
    let mut coordinate_pairs: Vec<[f64; 2]> = Vec::new();



    while let Ok(event) = json_reader.read_event(&mut buffer) {
        match event {
            JsonEvent::ObjectKey(k) if k == "coordinates" => {
                // Coordinate capture.
                let mut depth = 0;
                let mut dimensions = 0;
                let mut coord_pair_buf = [0f64, 0f64];
                while let Ok(event) = json_reader.read_event(&mut buffer) {
                    // coord capture logic...
                    match event {
                        JsonEvent::StartArray => {
                            depth += 1; // Iterate depth.
                        },
                        JsonEvent::EndArray => {
                            depth -= 1; // Decrement
                            if dimensions != 0 {
                                coordinate_pairs.push(coord_pair_buf.clone());
                            }
                            dimensions = 0;
                        },
                        JsonEvent::Number(num_str) => {
                            if dimensions < 2 {
                                coord_pair_buf[dimensions] = match num_str.parse::<f64>() {
                                    Ok(v) => v,
                                    Err(e) => {
                                        eprintln!("Unparsable number string GEOJSON file! {e:?}");
                                        return Err(GeoJSONErrorState::UnparsableCoordinate(num_str.to_string()));
                                    }
                                }
                            }
                            dimensions += 1;

                        }
                        _ => {}
                    }
                    if depth == 0 {
                        break; // Terminate loop.
                    }
                }

            },
            JsonEvent::Eof => break,
            _ => {}
    }}

    let boundaries = get_boundaries(coordinate_pairs);
    println!("Boundaries: {boundaries:?}");
    return Ok(Box::new(GeoJSONRegion {
        top_left: boundaries.1,
        bottom_right: boundaries.0,
    }))
}