use crate::parsing::kml::KMLErrorState::{NotEnoughGeoData, UnexpectedFormat};
use crate::spatial::Coordinate;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;
use xml::reader::{EventReader, XmlEvent};

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
    pub top_right: Coordinate,
    pub bottom_left: Coordinate,
}
#[derive(Debug)]
pub struct KMLMetadata {
    pub region: KMLRegion,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug)]
pub enum KMLErrorState {
    UnexpectedFormat(String),
    NotEnoughGeoData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KMLMap {
    pub(crate) path: PathBuf,
}

impl Display for KMLErrorState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UnexpectedFormat(s) => format!("UnexpectedFormatError: {s}"),
                NotEnoughGeoData =>
                    "Not enough geographic data within the file to establish a boundary!"
                        .to_string(),
            }
        )
    }
}
impl Error for KMLErrorState {}

pub fn parse_kml(reader: &mut BufReader<File>) -> Result<KMLMetadata, KMLErrorState> {
    // Initialise Event iterator, as well as coordinate buffer.
    let mut reader = EventReader::new(reader).into_iter();
    let tags = vec![("Filetype".to_string(), "KML".to_string())];
    let mut coordinates: Vec<(f64, f64)> = vec![];

    while let Some(Ok(event)) = reader.next() {
        // Capture events until file over.
        match event {
            XmlEvent::StartElement { name, .. } if name.local_name == "coordinates" => {
                // When Coordinate element starts...
                while let Some(Ok(event)) = reader.next() {
                    // Start capturing all events until Coordinate element ends.
                    match event {
                        XmlEvent::Characters(_0) => {
                            // While capturing, get all raw chars.
                            // Conform data into coordinate pairs...
                            let _0 = _0.replace("\n", "");
                            let coordinate_pairs = _0.split_whitespace(); // Split by whitespace, each coord set is space seperated.
                            for coordinate_pair in coordinate_pairs {
                                let coordinate_strs: Vec<&str> =
                                    coordinate_pair.split(",").collect();
                                if coordinate_strs.len() < 2 {
                                    return Err(UnexpectedFormat(format!(
                                        "Expected coordinate pair of len 2, got: {:?}",
                                        coordinate_strs
                                    )));
                                }
                                coordinates.push((
                                    match f64::from_str(coordinate_strs[0]) {
                                        Ok(v) => v,
                                        Err(e) => {
                                            return Err(UnexpectedFormat(format!("Failed to parse floating point coord: {} with err: {:?}", coordinate_strs[0], e)));
                                        }
                                    }, match f64::from_str(coordinate_strs[1]) {
                                        Ok(v) => v,
                                        Err(e) => {
                                            return Err(UnexpectedFormat(format!("Failed to parse floating point coord: {} with err: {:?}", coordinate_strs[1], e)));
                                        }
                                    }
                                ));
                            }
                        }
                        XmlEvent::EndElement { name } if name.local_name == "coordinates" => break, // Handle end of coordinate element
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
    return Ok(KMLMetadata {
        region: KMLRegion {
            bottom_left,
            top_right,
        },
        tags,
    }); // Return region defined by file.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use std::io::{Seek, Write};
    use tempfile::tempfile;

    #[test]
    fn test_parse_kml_with_tempfile() {
        // Create a mock KML file content
        let kml_data = r#"
            <kml>
                <Document>
                    <Placemark>
                        <Point>
                            <coordinates>-122.0822035425683,37.42228990140251,0</coordinates>
                        </Point>
                    </Placemark>
                    <Placemark>
                        <Point>
                            <coordinates>-123.0822035425683,38.42228990140251,0</coordinates>
                        </Point>
                    </Placemark>
                </Document>
            </kml>
        "#;

        // Create a temporary file and write the KML data into it
        let mut file = tempfile().unwrap();
        write!(file, "{}", kml_data).unwrap();
        file.flush().unwrap();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        // Use BufReader to read the temporary file
        let mut reader = BufReader::new(file);

        // Call the parse_kml function
        let result = parse_kml(&mut reader);
        assert!(result.is_ok());

        // Check if the parsed result matches the expected values
        let kml_meta = result.unwrap();
        assert_eq!(
            kml_meta.region.bottom_left,
            (-123.0822035425683, 37.42228990140251)
        );
        assert_eq!(
            kml_meta.region.top_right,
            (-122.0822035425683, 38.42228990140251)
        );
    }
    #[test]
    fn test_empty_kml() {
        let kml_data = r#"<kml></kml>"#;
        let mut file = tempfile().unwrap();
        write!(file, "{}", kml_data).unwrap();
        file.flush().unwrap();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        let mut reader = BufReader::new(file);
        let result = parse_kml(&mut reader);
        assert!(matches!(result, Err(NotEnoughGeoData)));
    }

    #[test]
    fn test_parse_kml_with_invalid_coordinates_format() {
        let kml_data = r#"
        <kml>
            <Document>
                <Placemark>
                    <Point>
                        <coordinates>abc,xyz,0</coordinates>
                    </Point>
                </Placemark>
            </Document>
        </kml>
    "#;
        let mut file = tempfile().unwrap();
        write!(file, "{}", kml_data).unwrap();
        file.flush().unwrap();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut reader = BufReader::new(file);
        let result = parse_kml(&mut reader);

        assert!(matches!(result, Err(UnexpectedFormat(_))));
    }

    #[test]
    fn test_parse_kml_empty_coordinates() {
        let kml_data = r#"
        <kml>
            <Document>
                <Placemark>
                    <Point>
                        <coordinates></coordinates>
                    </Point>
                </Placemark>
            </Document>
        </kml>
    "#;
        let mut file = tempfile().unwrap();
        write!(file, "{}", kml_data).unwrap();
        file.flush().unwrap();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut reader = BufReader::new(file);
        let result = parse_kml(&mut reader);

        assert!(matches!(result, Err(NotEnoughGeoData)));
    }

    #[test]
    fn test_parse_kml_multiple_placemarks() {
        let kml_data = r#"
        <kml>
            <Document>
                <Placemark>
                    <Point>
                        <coordinates>-122.0,37.0,0</coordinates>
                    </Point>
                </Placemark>
                <Placemark>
                    <Point>
                        <coordinates>-123.0,38.0,0</coordinates>
                    </Point>
                </Placemark>
                <!-- More Placemark elements can be added as needed -->
            </Document>
        </kml>
    "#;
        let mut file = tempfile().unwrap();
        write!(file, "{}", kml_data).unwrap();
        file.flush().unwrap();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut reader = BufReader::new(file);
        let result = parse_kml(&mut reader).unwrap();

        // Verify if the region boundary defined by all the Placemark elements is correctly calculated
        assert_eq!(result.region.bottom_left, (-123.0, 37.0));
        assert_eq!(result.region.top_right, (-122.0, 38.0));
    }

    #[test]
    fn test_parse_kml_insufficient_coordinates() {
        let kml_data = r#"
        <kml>
            <Document>
                <Placemark>
                    <Point>
                        <coordinates>-122.0822035425683,37.42228990140251,0</coordinates>
                    </Point>
                </Placemark>
            </Document>
        </kml>
    "#;
        let mut file = tempfile().unwrap();
        write!(file, "{}", kml_data).unwrap();
        file.flush().unwrap();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut reader = BufReader::new(file);
        let result = parse_kml(&mut reader);

        assert!(result.is_ok());
    }
}
