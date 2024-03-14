use std::fs::File;
use std::io::{BufReader,Read};
use std::path::PathBuf;
use crate::spatial::Coordinate;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use crate::parsing::mbtiles::{MBTilesMetaData, MBTilesRegion};

pub struct GPKG {
    min_x:f64,
    min_y:f64,
    max_x:f64,
    max_y:f64,
}

#[derive(Debug)]
pub struct GPKGRegion {
    pub top_left: (f64,f64),
    pub bottom_right: (f64,f64),
}

#[derive(Debug)]
pub struct GPKGMetaData {
    pub region: GPKGRegion,
    pub tags: Vec<(String, String)>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPKGMap {
    pub(crate) path: PathBuf
}

pub fn parse_gpkg(filepath: &str) ->Result<GPKGMetaData> {
    //Tags for metadata
    let mut tags = vec![("Filetype".to_string(), "GPKG".to_string())];

    let conn_result = Connection::open(filepath);

    //Some error handling
    let conn = match conn_result {
        Ok(conn) => conn,
        Err(error) => panic!("Problem Occurred when connecting to DB file: {:?}",error),
    };

    let mut stmt_result = conn.prepare("SELECT min_x,min_y,max_x,max_y FROM gpkg_contents");

    let mut stmt = match stmt_result {
        Ok(stmt) => stmt,
        Err(error) => panic!("Problem Occurred when querying the DB: {:?}", error),
    };

    let coords_iter_result = stmt.query_map([], |row| {
        Ok(GPKG {
            min_x: row.get(0)?,
            min_y: row.get(1)?,
            max_x: row.get(2)?,
            max_y: row.get(3)?,
        })
    });

    let coords_iter = match coords_iter_result {
        Ok(coords_iter) => coords_iter,
        Err(error) => panic!("Problem Occurred when iterating through query results: {:?}", error),
    };

    let mut top_left_result: (f64,f64) = (0.0,0.0);
    let mut bottom_right_result: (f64,f64) = (0.0,0.0);

    //Iterate through results, should only be one set of coords
    for coords in coords_iter {
        let temp = match coords {
            Ok(temp) => temp,
            Err(error) => panic!("Problem Occurred when retrieving coordinates: {:?}", error),
        };

        top_left_result = (temp.min_x,temp.max_y);
        bottom_right_result = (temp.max_x,temp.min_y);
    }

    return Ok(GPKGMetaData {
        region: GPKGRegion{
            top_left:top_left_result,
            bottom_right:bottom_right_result ,
        },
        tags
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{params, Connection};
    use tempfile::NamedTempFile;

    // Creates a temporary GPKG file for testing
    fn create_test_gpkg() -> NamedTempFile {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = Connection::open(temp_file.path()).unwrap();

        // Create gpkg_contents table and insert test coordinates
        conn.execute(
            "CREATE TABLE gpkg_contents (min_x REAL, min_y REAL, max_x REAL, max_y REAL);",
            [],
        ).unwrap();

        conn.execute(
            "INSERT INTO gpkg_contents (min_x, min_y, max_x, max_y) VALUES (?1, ?2, ?3, ?4)",
            params![10.1, 20.2, 30.3, 40.4], // Sample bounds
        ).unwrap();

        temp_file
    }

    #[test]
    fn test_parse_gpkg() {
        let temp_file = create_test_gpkg();
        let temp_file_path = temp_file.path().to_str().unwrap();

        let metadata = parse_gpkg(temp_file_path).unwrap();

        // Asserts the top_left and bottom_right coordinates are as expected
        assert_eq!(metadata.region.top_left, (10.1, 40.4));
        assert_eq!(metadata.region.bottom_right, (30.3, 20.2));

        // Checks if the tags contain the "Filetype" => "GPKG" entry
        assert!(metadata.tags.iter().any(|(key, value)| key == "Filetype" && value == "GPKG"));
    }

    #[test]//Panic?
    #[should_panic]
    fn test_parse_gpkg_empty() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_path = temp_file.path().to_str().unwrap();

        let metadata = parse_gpkg(temp_file_path);

        assert!(metadata.is_err());
    }
    #[test]
    fn test_parse_gpkg_multiple_entries() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = Connection::open(temp_file.path()).unwrap();

        conn.execute(
            "CREATE TABLE gpkg_contents (min_x REAL, min_y REAL, max_x REAL, max_y REAL);",
            [],
        ).unwrap();

        conn.execute("INSERT INTO gpkg_contents (min_x, min_y, max_x, max_y) VALUES (1.0, 2.0, 3.0, 4.0), (5.0, 6.0, 7.0, 8.0);", []).unwrap();

        let temp_file_path = temp_file.path().to_str().unwrap();
        let metadata = parse_gpkg(temp_file_path).unwrap();

        assert_eq!(metadata.region.top_left, (5.0,8.0));
        assert_eq!(metadata.region.bottom_right, (7.0, 6.0));
    }

}
