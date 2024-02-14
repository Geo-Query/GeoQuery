use std::fs::File;
use std::io::{BufReader,Read};
use crate::spatial::Coordinate;
use rusqlite::{Connection, Result};
//use crate::parsing::mbtiles::MBTilesErrorState::{UnexpectedFormat,FailedQuery};

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

pub fn get_boundaries(coordinates: Vec<Coordinate>) -> (Coordinate,Coordinate) {

}

pub fn parse_mbtiles(filepath: &str ) -> Result<MBTilesMetaData> {
    //Tags for metadata
    let mut tags = vec![("Filetype".to_string(), "MBTiles".to_string())];

    let conn_result = Connection::open(filepath);

    //Some error handling
    let conn = match conn_result {
        Ok(conn) => conn,
        Err(error) => panic!("Problem Occurred when connecting to DB file: {:?}",error),
    };

    //Add error handling for Unsupported Version MBTiles 1.0

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

        //bottom_left_result = (values[1], values[0]);
        //top_right_result = (values[3], values[2]);

        //Goes lat then long

        bottom_right_result = (values[1], values[2]);
        top_left_result = (values[3], values[0]);

    }

    return Ok(MBTilesMetaData {
        region: MBTilesRegion{
                top_left: top_left_result,
                bottom_right: bottom_right_result,
        },
        tags
    });
}