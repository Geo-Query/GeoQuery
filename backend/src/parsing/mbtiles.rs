use std::fs::File;
use std::io::BufReader;
use crate::spatial::Coordinate;
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct MBTilesRegion {
    pub bottom_left: Coordinate,
    pub top_right: Coordinate,
}

pub fn parse_mbtiles(reader: &mut BufReader<File>) -> Result<MBTilesRegion> {
    //Opens connection to MBTiles file(its just a sqlite db)
    let conn = Connection::open(reader).unwrap();

    //Prepares a query to return bounds of MBTiles using metadata table
    let mut stmt = conn.prepare("SELECT * FROM metadata WHERE name = 'bounds'").unwrap();

    let mut rows = stmt.query([]).unwrap();

    while let Some(row) = rows.next().unwrap() {
        let value: f64 = row.get("value").unwrap();

        //Mbtiles bounds coordinates are stored as left, bottom, right, top
        let values: Vec<&f64> = value.split(',').collect();
    }

    let (bottom_left, top_right) = ((value[1],values[0]),(value[3],value[2]));

    return Ok(MBTilesRegion {
        bottom_left,
        top_right
    });

}