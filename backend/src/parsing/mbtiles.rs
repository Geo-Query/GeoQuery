use std::fs::File;
use std::io::{BufReader,Read};
use crate::spatial::Coordinate;
use rusqlite::{Connection, Result};
use crate::parsing::mbtiles::MBTilesErrorState::{UnexpectedFormat,FailedQuery};

#[derive(Debug)]
pub struct MBTilesRegion {
    pub top_right: Coordinate,
    pub bottom_left: Coordinate,
}

//Get this to work
pub enum MBTilesErrorState {
    UnexpectedFormat(String),
    FailedQuery
}

pub fn parse_mbtiles(reader: &mut BufReader<File>) -> Result<MBTilesRegion> {
    //Opens connection to MBTiles file(its just a sqlite db)
    let mut filename = String::new();
    reader.read_to_string(&mut filename).expect("Cannot Read String");
    let conn = Connection::open(filename).unwrap();

    //Prepares a query to return bounds of MBTiles using metadata table
    let mut stmt_result = conn.prepare("SELECT * FROM metadata WHERE name = 'bounds'");

    //Add error handling here
    let mut stmt = match stmt_result {
        Ok(stmt) => stmt,
        Err(error) => panic!("Problem Occurred when querying the DB: {:?}",error),
        //Err(e) => return Err(FailedQuery),
    };

    let mut rows = stmt.query([]).unwrap();

    //Iterate through result rows of query
    while let Some(row) = rows.next().unwrap() {
        let value: String = row.get("value").unwrap();

        //Mbtiles bounds coordinates are stored as left, bottom, right, top
        let values: Vec<f64> = value
            .split(',')
            .map(|s| s.trim().parse::<f64>())
            .collect::<Result<_, _>>().unwrap();

        //Goes lat then long
        let bottom_left = (values[1], values[0]);
        let top_right = (values[3], values[2]);
    }


    return Ok(MBTilesRegion {
        top_right,
        bottom_left,
    });

}