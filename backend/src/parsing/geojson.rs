use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
//use std::io::BufReader;
use std::io::{BufReader, Write, Seek, SeekFrom};
use std::path::PathBuf;
use axum::extract::Path;
use crate::spatial::Coordinate;
use json_event_parser::{JsonReader, JsonEvent};
use serde::{Deserialize, Serialize};
use tempfile::tempfile;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GEOJSONMap {
    pub(crate) path: PathBuf
}

impl Display for GeoJSONErrorState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GeoJSONErrorState {
    // TODO: Add error descriptions!
}

#[derive(Debug, Clone)]
pub struct GeoJSONRegion {
    pub top_right: Coordinate,
    pub bottom_left: Coordinate
}

#[derive(Debug, Clone)]
pub struct GeoJSONMetaData {
    pub region: GeoJSONRegion,
    pub tags: Vec<(String, String)>
}

pub fn parse_geojson(reader: &mut BufReader<File>) -> Result<GeoJSONMetaData, GeoJSONErrorState> {
    let mut json_reader = JsonReader::from_reader(reader);
    let mut buffer = Vec::new();
    let mut coordinate_pairs: Vec<[f64; 2]> = Vec::new();
    let mut tags = vec![("Filetype".to_string(), "GEOJSON".to_string())];


    while let event = match json_reader.read_event(&mut buffer) {
        Ok(event) => event,
        Err(e) => {
            return Err(GeoJSONErrorState::InvalidJSON(Box::new(e)));
        }
    } {
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
                                        eprintln!("Unparsable number string in GEOJSON file! {e:?}");
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
            JsonEvent::Eof => {
                break;
            }
            _ => {}
    }}

    let boundaries = get_boundaries(coordinate_pairs);
    return Ok(GeoJSONMetaData {
        region: GeoJSONRegion {
            top_right: boundaries.1,
            bottom_left: boundaries.0,
        },
        tags
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempfile;
    use std::io::Write;
    use std::io::Seek;
    use std::io::BufReader;
    use std::io::SeekFrom;

    #[test]
    fn test_get_boundaries() {
        // Test coordinates for boundary calculation
        let coordinates = vec![
            [0.0, 0.0], // Bottom left corner
            [2.0, 2.0], // Top right corner
            [0.0, 2.0], // Top left corner
            [2.0, 0.0], // Bottom right corner
        ];
        let (bottom_left, top_right) = get_boundaries(coordinates);
        assert_eq!(bottom_left, (0.0, 0.0)); // Assert bottom left corner
        assert_eq!(top_right, (2.0, 2.0));   // Assert top right corner
    }

    #[test]
    fn test_parse_geojson_valid() {
        // Create a mock file or in-memory stream containing valid GeoJSON data
        // The example GeoJSON data should include a valid set of coordinates
        let geojson_data = br#"{
            "type": "Feature",
            "geometry": {
                "type": "Polygon",
                "coordinates": [[[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0], [0.0, 0.0]]]
            }
        }"#;
        let mut temp_file = tempfile().unwrap();
        temp_file.write_all(geojson_data).unwrap();
        temp_file.seek(SeekFrom::Start(0)).unwrap(); // Reset file pointer to the start

        let mut reader = BufReader::new(temp_file);
        let result = parse_geojson(&mut reader);
        assert!(result.is_ok()); // Assert that parsing was successful
    } 
}

