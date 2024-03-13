use std::fs::File;
use std::io::{BufReader,Read};
use std::path::PathBuf;
use crate::spatial::Coordinate;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct MBTilesRegion {
    pub top_left: Coordinate,
    pub bottom_right: Coordinate,
}

#[derive(Debug)]
pub struct MBTilesMetaData {
    pub region: MBTilesRegion,
    pub tags: Vec<(String, String)>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MBTilesMap {
    pub(crate) path: PathBuf
}

pub fn parse_mbtiles(filepath: &str ) -> Result<MBTilesMetaData> {
    //Tags for metadata
    let mut tags = vec![("Filetype".to_string(), "MBTILES".to_string())];

    let conn_result = Connection::open(filepath);

    //Some error handling
    let conn = match conn_result {
        Ok(conn) => conn,
        Err(error) => panic!("Problem Occurred when connecting to DB file: {:?}",error),
    };



    //Prepares a query to return bounds of MBTiles using metadata table
    let stmt_result = conn.prepare("SELECT * FROM metadata WHERE name = 'bounds'");


    //Some error handling
    let mut stmt = match stmt_result {
        Ok(stmt) => stmt,
        Err(error) => panic!("Problem Occurred when querying the DB: {:?}",error),
    };

    let mut rows = stmt.query([])?;

    let mut top_left_result: (f64,f64) = (0.0,0.0);
    let mut bottom_right_result: (f64,f64) = (0.0,0.0);

    //Iterate through result rows of query
    while let Some(row) = rows.next().unwrap() {
        let value: String = row.get("value").unwrap();

        //Mbtiles bounds coordinates are stored as left, bottom, right, top
        let values_result = value.split(',').map(|s| s.trim().parse::<f64>()).collect::<Result<_, _>>();

        //Some error handling
        let values:Vec<f64> = match values_result {
            Ok(values) => values,
            Err(error) => panic!("Problem Occurred when parsing coordinates to float64: {:?}",error)
        };

        //Goes long then lat

        bottom_right_result = (values[2], values[1]);
        top_left_result = (values[0], values[3]);

    }

    return Ok(MBTilesMetaData {
        region: MBTilesRegion{
                top_left: top_left_result,
                bottom_right: bottom_right_result,
        },
        tags
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{params, Connection};
    use tempfile::NamedTempFile;
    use std::fs;

    fn create_test_mbtiles() -> NamedTempFile {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = Connection::open(temp_file.path()).unwrap();

        // Creates the metadata table and inserts test bounds
        conn.execute(
            "CREATE TABLE metadata (name TEXT, value TEXT);",
            [],
        ).unwrap();

        conn.execute(
            "INSERT INTO metadata (name, value) VALUES (?1, ?2), (?3, ?4)",
            params![
                "bounds", "10.1,20.2,30.3,40.4", // left, bottom, right, top
                "name", "Test MBTiles"
            ],
        ).unwrap();

        temp_file
    }

    #[test]
    fn test_parse_mbtiles() {
        let temp_file = create_test_mbtiles();
        let temp_file_path = temp_file.path().to_str().unwrap();

        let metadata = parse_mbtiles(temp_file_path).unwrap();

        assert_eq!(metadata.region.top_left, (10.1, 40.4));
        assert_eq!(metadata.region.bottom_right, (30.3, 20.2));
        assert!(metadata.tags.iter().any(|(key, value)| key == "Filetype" && value == "MBTILES"));
    }
}

