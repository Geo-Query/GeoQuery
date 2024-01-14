use std::fs::File;
use std::io::BufReader;
use crate::spatial::Coordinate;
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct MBTilesRegion {
    pub top_right: Coordinate,
    pub bottom_left: Coordinate,
}

pub enum MBTilesErrorState {
    UnexpectedFormat(String),
}

pub fn parse_mbtiles(reader: &mut BufReader<File>) -> Result<MBTilesRegion> {
    //Opens connection to MBTiles file(its just a sqlite db)
    let conn = Connection::open(reader).unwrap();

    //Prepares a query to return bounds of MBTiles using metadata table
    let mut stmt = conn.prepare("SELECT * FROM metadata WHERE name = 'bounds'").unwrap();

    //Add error handling here

    let mut rows = stmt.query([]).unwrap();

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