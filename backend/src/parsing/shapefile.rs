use crate::spatial::Region;
use proj4rs::proj::ProjType;
use proj4rs::Proj;
use proj4wkt::wkt_to_projstring;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, ErrorKind, Read};
use std::path::PathBuf;
use tracing::{event, Level};

pub trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl FromBytes for f64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        let bytes: [u8; 8] = bytes.try_into().unwrap();
        return f64::from_le_bytes(bytes);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeFileMap {
    pub shp: PathBuf,
    pub prj: Option<PathBuf>,
    pub tfw: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeFileMetaData {
    pub region: Region,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug)]
pub enum ShapeFileErrorKind {
    UnexpectedMagicNumber([u8; 4]),
}

impl Display for ShapeFileErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for ShapeFileErrorKind {}

#[derive(Debug)]
pub struct ShapeFileHeader {
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
}

pub fn parse_header(buffer: &[u8]) -> Result<ShapeFileHeader, Box<dyn Error>> {
    assert_eq!(buffer.len(), 100); // Assert valid slice! Runtime check!
    if buffer[0..4] != [0, 0, 39, 10] {
        return Err(ShapeFileErrorKind::UnexpectedMagicNumber([
            buffer[0], buffer[1], buffer[2], buffer[3],
        ])
        .into());
    }
    let x_min = f64::from_bytes(&buffer[36..44]);
    let y_min = f64::from_bytes(&buffer[44..52]);
    let x_max = f64::from_bytes(&buffer[52..60]);
    let y_max = f64::from_bytes(&buffer[60..68]);

    return Ok(ShapeFileHeader {
        x_min,
        y_min,
        x_max,
        y_max,
    });
}

pub fn parse_shapefile(
    shp_reader: &mut BufReader<File>,
    prj_reader: Option<&mut BufReader<File>>,
) -> Result<ShapeFileMetaData, Box<dyn Error>> {
    let to_proj = Proj::from_proj_string(crs_definitions::EPSG_4326.proj4)
        .expect("FAILED TO BUILD DEFAULT PROJ!");
    let tags = vec![("Filetype".to_string(), "SHAPEFILE".to_string())];
    let mut header_buf = [0u8; 100];
    shp_reader.read_exact(&mut header_buf)?;
    let header = parse_header(&header_buf)?;

    if let Some(prj_reader) = prj_reader {
        let mut prj_content = String::new();
        prj_reader.read_to_string(&mut prj_content)?;
        let proj = Proj::from_proj_string(wkt_to_projstring(prj_content.as_str())?.as_str())?;
        let (mut top_left, mut bottom_right) = match proj.projection_type() {
            ProjType::Latlong => (
                (header.x_min.to_radians(), header.y_max.to_radians()),
                (header.x_max.to_radians(), header.y_min.to_radians()),
            ),
            ProjType::Other => ((header.x_min, header.y_max), (header.x_max, header.y_min)),
            ProjType::Geocentric => {
                event!(Level::ERROR, "Unsupported projection! From GEOCENTRIC!");
                event!(
                    Level::ERROR,
                    "Please contact developer, and send file content for implementation."
                );
                panic!();
            }
        };

        event!(
            Level::INFO,
            "Applying Projection to {top_left:?} and {bottom_right:?}"
        );
        proj4rs::transform::transform(&proj, &to_proj, &mut top_left)?;
        proj4rs::transform::transform(&proj, &to_proj, &mut bottom_right)?;
        event!(Level::INFO, "Parsed shapefile and applied projection!");

        return Ok(ShapeFileMetaData {
            region: Region {
                top_left: (top_left.0.to_degrees(), top_left.1.to_degrees()),
                bottom_right: (bottom_right.0.to_degrees(), bottom_right.1.to_degrees()),
            },
            tags,
        });
    } else {
        event!(
            Level::WARN,
            "Shapefile without accompanying projection found!"
        );
        event!(
            Level::WARN,
            "This is a forseen error, and we will assume that the CRS is EPSG:4326!"
        );
        event!(
            Level::WARN,
            "However, this might be incorrect! If you encounter inaccuracies in shapefile"
        );
        event!(
            Level::WARN,
            "Please contact the developers, and attach the unhandled file!"
        );
        // TODO: Add a config option to disable this behaviour!
        let mut top_left = (header.x_min, header.y_max);
        let mut bottom_right = (header.x_max, header.y_min);
        return Ok(ShapeFileMetaData {
            region: Region {
                top_left,
                bottom_right,
            },
            tags,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::io::{Seek, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn test_from_bytes_f64() {
        let bytes = 42.42f64.to_le_bytes();
        let result = f64::from_bytes(&bytes);
        assert!((result - 42.42).abs() < f64::EPSILON);
    }
    #[test]
    fn test_parse_header_valid() {
        let mut header_bytes = vec![0; 100];
        header_bytes[0] = 0;
        header_bytes[1] = 0;
        header_bytes[2] = 39;
        header_bytes[3] = 10;
        // Populate with test coordinate data
        let x_min_bytes = (-123.456f64).to_le_bytes();
        header_bytes[36..44].copy_from_slice(&x_min_bytes);

        let header = parse_header(&header_bytes).unwrap();
        assert_eq!(header.x_min, -123.456);
    }

    #[test]
    fn test_parse_header_invalid_magic_number() {
        let header_bytes = vec![0; 100];
        assert!(parse_header(&header_bytes).is_err());
    }
    fn create_temp_shapefile(
        header_bytes: &[u8],
        prj_content: Option<&str>,
    ) -> (BufReader<File>, Option<BufReader<File>>) {
        let mut temp_shp = NamedTempFile::new().unwrap();
        temp_shp.write_all(header_bytes).unwrap();
        temp_shp.as_file().sync_all().unwrap();
        let shp_reader = BufReader::new(temp_shp.reopen().unwrap());

        let prj_reader = if let Some(content) = prj_content {
            let mut temp_prj = NamedTempFile::new().unwrap();
            temp_prj.write_all(content.as_bytes()).unwrap();
            temp_prj.as_file().sync_all().unwrap();
            Some(BufReader::new(temp_prj.reopen().unwrap()))
        } else {
            None
        };

        (shp_reader, prj_reader)
    }

    #[test]
    fn test_reading_shapefile_with_io_error() {
        let temp_shp = NamedTempFile::new().unwrap();
        let mut shp_reader = BufReader::new(temp_shp.reopen().unwrap());

        // Write invalid content to trigger an error during the read process
        let invalid_content = vec![0; 10];
        shp_reader.get_mut().write_all(&invalid_content).unwrap();
        shp_reader.get_mut().sync_all().unwrap();
        shp_reader.get_mut().seek(io::SeekFrom::Start(0)).unwrap();

        let result = parse_shapefile(&mut shp_reader, None);
        assert!(result.is_err());
    }
    #[test]
    fn test_parse_shapefile_with_invalid_prj_content() {
        let header_bytes = [0; 100];
        let prj_content = "INVALID_PROJECTION"; // Invalid PRJ content
        let (mut shp_reader, mut prj_reader) =
            create_temp_shapefile(&header_bytes, Some(prj_content));

        let result = parse_shapefile(&mut shp_reader, prj_reader.as_mut());
        assert!(result.is_err());
    }
    #[test]
    fn test_parse_empty_shapefile() {
        let header_bytes = []; // Empty header size
        let (mut shp_reader, mut prj_reader) = create_temp_shapefile(&header_bytes, None);

        let result = parse_shapefile(&mut shp_reader, prj_reader.as_mut());
        assert!(result.is_err());
    }
    #[test]
    fn test_parse_shapefile_with_large_coordinates() {
        let mut header_bytes = vec![0; 100];
        header_bytes[0] = 0;
        header_bytes[1] = 0;
        header_bytes[2] = 39;
        header_bytes[3] = 10;
        // Set abnormally large coordinate values
        let large_value = 1e40f64.to_le_bytes();
        header_bytes[36..44].copy_from_slice(&large_value);
        header_bytes[52..60].copy_from_slice(&large_value);

        let (mut shp_reader, mut prj_reader) = create_temp_shapefile(&header_bytes, None);

        let result = parse_shapefile(&mut shp_reader, prj_reader.as_mut());
        assert!(result.is_ok(), "Should handle large coordinates gracefully");
    }

    #[test]
    fn test_parse_shapefile_with_incorrect_header_size() {
        let header_bytes = [0; 90]; // Incorrect header size
        let (mut shp_reader, mut prj_reader) = create_temp_shapefile(&header_bytes, None);

        let result = parse_shapefile(&mut shp_reader, prj_reader.as_mut());
        assert!(result.is_err());
    }
}
