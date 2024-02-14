use std::fs::File;
use std::io::{BufReader,Read};
use crate::spatial::Coordinate;
use rusqlite::{Connection, Result};
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